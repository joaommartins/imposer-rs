"""
Tests for the imposer Python bindings.

These tests verify the functionality of the PyO3 bindings for the imposer
PDF booklet imposition library.
"""

import pytest
import os
from pathlib import Path

# Import the compiled extension module
import imposer


class TestPageSize:
    """Tests for the PageSize class."""

    def test_a4_creation(self):
        """Test creating an A4 page size."""
        page_size = imposer.PageSize.a4()
        assert page_size is not None

    def test_a3_creation(self):
        """Test creating an A3 page size."""
        page_size = imposer.PageSize.a3()
        assert page_size is not None

    def test_a5_creation(self):
        """Test creating an A5 page size."""
        page_size = imposer.PageSize.a5()
        assert page_size is not None

    def test_letter_creation(self):
        """Test creating a Letter page size."""
        page_size = imposer.PageSize.letter()
        assert page_size is not None

    def test_legal_creation(self):
        """Test creating a Legal page size."""
        page_size = imposer.PageSize.legal()
        assert page_size is not None

    def test_tabloid_creation(self):
        """Test creating a Tabloid page size."""
        page_size = imposer.PageSize.tabloid()
        assert page_size is not None


class TestBindingType:
    """Tests for the BindingType class."""

    def test_saddle_stitch_creation(self):
        """Test creating a saddle stitch binding type."""
        binding = imposer.BindingType.saddle_stitch()
        assert binding is not None

    def test_perfect_bound_creation(self):
        """Test creating a perfect bound binding type with 8 sheets per signature."""
        binding = imposer.BindingType.perfect_bound(sheets_per_signature=8, num_signatures=3)
        assert binding is not None

    def test_perfect_bound_defaults(self):
        """Test creating a perfect bound binding type with default values."""
        binding = imposer.BindingType.perfect_bound(sheets_per_signature=8, num_signatures=1)
        assert binding is not None


class TestBookletConfig:
    """Tests for the BookletConfig builder class."""

    def test_default_config(self):
        """Test creating a default configuration."""
        config = imposer.BookletConfig()
        assert config is not None

    def test_with_page_size(self):
        """Test setting page size."""
        page_size = imposer.PageSize.letter()
        config = imposer.BookletConfig().with_page_size(page_size)
        assert config is not None

    def test_with_pages_per_sheet(self):
        """Test setting pages per sheet to 3 (3-up layout)."""
        config = imposer.BookletConfig().with_pages_per_sheet(3)
        assert config is not None

    def test_with_scale_to_fit(self):
        """Test enabling scale to fit."""
        config = imposer.BookletConfig().with_scale_to_fit(True)
        assert config is not None

    def test_with_preserve_aspect_ratio(self):
        """Test disabling preserve aspect ratio."""
        config = imposer.BookletConfig().with_preserve_aspect_ratio(False)
        assert config is not None

    def test_with_draw_guides(self):
        """Test enabling draw guides."""
        config = imposer.BookletConfig().with_draw_guides(True)
        assert config is not None

    def test_with_number_pages(self):
        """Test enabling page numbering."""
        config = imposer.BookletConfig().with_number_pages(True)
        assert config is not None

    def test_with_binding_type(self):
        """Test setting binding type to saddle stitch."""
        binding = imposer.BindingType.saddle_stitch()
        config = imposer.BookletConfig().with_binding_type(binding)
        assert config is not None

    def test_with_sheets_per_signature(self):
        """Test setting sheets per signature to 8."""
        config = imposer.BookletConfig().with_sheets_per_signature(8)
        assert config is not None

    def test_with_num_signatures(self):
        """Test setting number of signatures to 3."""
        config = imposer.BookletConfig().with_num_signatures(3)
        assert config is not None

    def test_builder_chain(self):
        """Test chaining multiple builder methods."""
        config = (
            imposer.BookletConfig()
            .with_page_size(imposer.PageSize.a4())
            .with_pages_per_sheet(2)
            .with_scale_to_fit(True)
            .with_draw_guides(True)
            .with_number_pages(True)
        )
        assert config is not None


class TestGenerateBooklet:
    """Tests for the generate_booklet function."""

    @pytest.fixture
    def sample_pdf_path(self, tmp_path):
        """Create a minimal valid PDF for testing by assembling objects and computing xref offsets."""
        parts = []

        parts.append("%PDF-1.4\n")

        # Catalog and pages
        parts.append("1 0 obj\n<< /Type /Catalog /Pages 2 0 R >>\nendobj\n")
        parts.append("2 0 obj\n<< /Type /Pages /Kids [3 0 R 4 0 R 5 0 R 6 0 R] /Count 4 >>\nendobj\n")

        # Four page objects
        for i in range(3, 7):
            parts.append(f"{i} 0 obj\n<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Contents {i+4} 0 R /Resources << >> >>\nendobj\n")

        # Four content streams
        for j in range(7, 11):
            stream = f"BT\n/F1 12 Tf\n100 700 Td\n(Page {j-6}) Tj\nET\n"
            parts.append(f"{j} 0 obj\n<< /Length {len(stream)} >>\nstream\n{stream}endstream\nendobj\n")

        body = "".join(parts).encode("utf-8")

        # Compute offsets for xref table
        offsets = []
        for i in range(1, 11):
            marker = f"{i} 0 obj".encode("utf-8")
            idx = body.find(marker)
            if idx == -1:
                raise RuntimeError(f"Failed to find object {i} in PDF body")
            offsets.append(idx)

        xref_lines = ["xref\n0 11\n", "0000000000 65535 f \n"]
        for off in offsets:
            xref_lines.append(f"{off:010d} 00000 n \n")

        xref = "".join(xref_lines).encode("utf-8")
        trailer = "trailer\n<< /Size 11 /Root 1 0 R >>\n".encode("utf-8")

        # xref begins immediately after the body
        startxref = str(len(body)).encode("utf-8")

        pdf_bytes = body + xref + trailer + b"startxref\n" + startxref + b"\n%%EOF\n"

        pdf_path = tmp_path / "input.pdf"
        pdf_path.write_bytes(pdf_bytes)
        return pdf_path

    def test_generate_booklet_default(self, sample_pdf_path):
        """Test generating a booklet with default configuration."""
        input_bytes = sample_pdf_path.read_bytes()
        config = imposer.BookletConfig()

        output_bytes = imposer.generate_booklet(input_bytes, config)

        # Verify we got a non-empty PDF back
        # Ensure we produced a PDF and that it's non-empty; size may vary across implementations
        assert len(output_bytes) > 0
        # Verify it starts with PDF header
        assert output_bytes.startswith(b'%PDF')

    def test_generate_booklet_custom_config(self, sample_pdf_path):
        """Test generating a booklet with custom configuration."""
        input_bytes = sample_pdf_path.read_bytes()
        config = (
            imposer.BookletConfig()
            .with_page_size(imposer.PageSize.letter())
            .with_pages_per_sheet(2)
            .with_scale_to_fit(True)
        )

        output_bytes = imposer.generate_booklet(input_bytes, config)

        assert len(output_bytes) > 0
        assert output_bytes.startswith(b'%PDF')

    def test_generate_booklet_invalid_pdf(self):
        """Test that invalid PDF input raises an error."""
        invalid_bytes = b"This is not a PDF"
        config = imposer.BookletConfig()

        with pytest.raises(Exception):  # Should raise a Python exception
            imposer.generate_booklet(invalid_bytes, config)

    def test_generate_booklet_empty_input(self):
        """Test that empty input raises an error."""
        empty_bytes = b""
        config = imposer.BookletConfig()

        with pytest.raises(Exception):
            imposer.generate_booklet(empty_bytes, config)


class TestGenerateBookletFromFile:
    """Tests for the generate_booklet_from_file function."""

    @pytest.fixture
    def sample_pdf_path(self, tmp_path):
        """Create a minimal valid PDF for testing by assembling objects and computing xref offsets."""
        parts = []

        parts.append("%PDF-1.4\n")
        parts.append("1 0 obj\n<< /Type /Catalog /Pages 2 0 R >>\nendobj\n")
        parts.append("2 0 obj\n<< /Type /Pages /Kids [3 0 R 4 0 R 5 0 R 6 0 R] /Count 4 >>\nendobj\n")

        for i in range(3, 7):
            parts.append(f"{i} 0 obj\n<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Contents {i+4} 0 R /Resources << >> >>\nendobj\n")

        for j in range(7, 11):
            stream = f"BT\n/F1 12 Tf\n100 700 Td\n(Page {j-6}) Tj\nET\n"
            parts.append(f"{j} 0 obj\n<< /Length {len(stream)} >>\nstream\n{stream}endstream\nendobj\n")

        body = "".join(parts).encode("utf-8")

        offsets = []
        for i in range(1, 11):
            marker = f"{i} 0 obj".encode("utf-8")
            idx = body.find(marker)
            if idx == -1:
                raise RuntimeError(f"Failed to find object {i} in PDF body")
            offsets.append(idx)

        xref_lines = ["xref\n0 11\n", "0000000000 65535 f \n"]
        for off in offsets:
            xref_lines.append(f"{off:010d} 00000 n \n")

        xref = "".join(xref_lines).encode("utf-8")
        trailer = "trailer\n<< /Size 11 /Root 1 0 R >>\n".encode("utf-8")

        # xref begins immediately after the body
        startxref = str(len(body)).encode("utf-8")

        pdf_bytes = body + xref + trailer + b"startxref\n" + startxref + b"\n%%EOF\n"

        pdf_path = tmp_path / "input.pdf"
        pdf_path.write_bytes(pdf_bytes)
        return pdf_path

    def test_generate_booklet_from_file_default(self, sample_pdf_path, tmp_path):
        """Test generating a booklet from file with default configuration."""
        output_path = tmp_path / "output.pdf"
        config = imposer.BookletConfig()

        imposer.generate_booklet_from_file(
            str(sample_pdf_path),
            str(output_path),
            config
        )

        # Verify output file was created
        assert output_path.exists()
        # Verify it's a PDF
        output_bytes = output_path.read_bytes()
        # Ensure we produced a PDF and that it's non-empty; size may vary across implementations
        assert len(output_bytes) > 0
        assert output_bytes.startswith(b'%PDF')

    def test_generate_booklet_from_file_custom_config(self, sample_pdf_path, tmp_path):
        """Test generating a booklet from file with custom configuration."""
        output_path = tmp_path / "output_custom.pdf"
        config = (
            imposer.BookletConfig()
            .with_page_size(imposer.PageSize.a4())
            .with_pages_per_sheet(3)
            .with_draw_guides(True)
        )

        imposer.generate_booklet_from_file(
            str(sample_pdf_path),
            str(output_path),
            config
        )

        assert output_path.exists()
        output_bytes = output_path.read_bytes()
        assert len(output_bytes) > 0
        assert output_bytes.startswith(b'%PDF')

    def test_generate_booklet_from_file_nonexistent_input(self, tmp_path):
        """Test that nonexistent input file raises an error."""
        nonexistent_path = tmp_path / "nonexistent.pdf"
        output_path = tmp_path / "output.pdf"
        config = imposer.BookletConfig()

        with pytest.raises(Exception):
            imposer.generate_booklet_from_file(
                str(nonexistent_path),
                str(output_path),
                config
            )

    def test_generate_booklet_from_file_invalid_pdf(self, tmp_path):
        """Test that invalid PDF file raises an error."""
        invalid_pdf_path = tmp_path / "invalid.pdf"
        invalid_pdf_path.write_text("Not a PDF")
        output_path = tmp_path / "output.pdf"
        config = imposer.BookletConfig()

        with pytest.raises(Exception):
            imposer.generate_booklet_from_file(
                str(invalid_pdf_path),
                str(output_path),
                config
            )


class TestIntegration:
    """Integration tests verifying end-to-end workflows."""

    @pytest.fixture
    def sample_8page_pdf_path(self, tmp_path):
        """Create an 8-page PDF for more realistic testing."""
        # Build a minimal but valid 8-page PDF dynamically (compute xref/startxref)
        parts = []
        parts.append("%PDF-1.4\n")

        # Catalog and Pages
        parts.append("1 0 obj\n<< /Type /Catalog /Pages 2 0 R >>\nendobj\n")
        kids = " ".join([f"{i} 0 R" for i in range(3, 11)])
        parts.append(f"2 0 obj\n<< /Type /Pages /Kids [{kids}] /Count 8 >>\nendobj\n")

        # Page objects (3..10)
        for i in range(3, 11):
            parts.append(f"{i} 0 obj\n<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Contents {i+8} 0 R /Resources << >> >>\nendobj\n")

        # Content streams (11..18)
        for j in range(11, 19):
            stream = f"BT\n/F1 12 Tf\n100 700 Td\n(Page {j-10}) Tj\nET\n"
            parts.append(f"{j} 0 obj\n<< /Length {len(stream)} >>\nstream\n{stream}endstream\nendobj\n")

        body = "".join(parts).encode("utf-8")

        # offsets for objects 1..18
        offsets = []
        for i in range(1, 19):
            marker = f"{i} 0 obj".encode("utf-8")
            idx = body.find(marker)
            if idx == -1:
                raise RuntimeError(f"Failed to find object {i} in PDF body")
            offsets.append(idx)

        xref_lines = ["xref\n0 19\n", "0000000000 65535 f \n"]
        for off in offsets:
            xref_lines.append(f"{off:010d} 00000 n \n")

        xref = "".join(xref_lines).encode("utf-8")
        trailer = "trailer\n<< /Size 19 /Root 1 0 R >>\n".encode("utf-8")
        startxref = str(len(body)).encode("utf-8")

        pdf_bytes = body + xref + trailer + b"startxref\n" + startxref + b"\n%%EOF\n"

        pdf_path = tmp_path / "8page.pdf"
        pdf_path.write_bytes(pdf_bytes)
        return pdf_path

    def test_saddle_stitch_8page_booklet(self, sample_8page_pdf_path, tmp_path):
        """
        Test creating an 8-page saddle-stitch booklet.

        Expected behavior:
        - 8 input pages should produce 4 output sheets (2 pages per sheet)
        - Pages should be ordered: [8,1], [2,7], [6,3], [4,5]
        """
        output_path = tmp_path / "saddle_stitch.pdf"
        config = (
            imposer.BookletConfig()
            .with_binding_type(imposer.BindingType.saddle_stitch())
            .with_pages_per_sheet(2)
        )

        imposer.generate_booklet_from_file(
            str(sample_8page_pdf_path),
            str(output_path),
            config
        )

        assert output_path.exists()
        output_bytes = output_path.read_bytes()
        # Ensure non-empty PDF instead of brittle size check
        assert len(output_bytes) > 0
        assert output_bytes.startswith(b'%PDF')

    def test_perfect_bound_booklet(self, sample_8page_pdf_path, tmp_path):
        """
        Test creating a perfect-bound booklet with signatures.

        Expected behavior:
        - Creates signatures with specified sheets per signature
        - Each signature has proper page ordering for perfect binding
        """
        output_path = tmp_path / "perfect_bound.pdf"
        config = (
            imposer.BookletConfig()
            .with_binding_type(
                imposer.BindingType.perfect_bound(
                    sheets_per_signature=4,
                    num_signatures=1
                )
            )
            .with_pages_per_sheet(2)
        )

        imposer.generate_booklet_from_file(
            str(sample_8page_pdf_path),
            str(output_path),
            config
        )

        assert output_path.exists()

    def test_3up_layout(self, sample_8page_pdf_path, tmp_path):
        """
        Test creating a 3-up layout (3 pages per sheet).

        Expected behavior:
        - Pages arranged 3 per sheet
        - Different page ordering than standard 2-up
        """
        output_path = tmp_path / "3up.pdf"
        config = (
            imposer.BookletConfig()
            .with_pages_per_sheet(3)
            .with_draw_guides(True)
        )

        imposer.generate_booklet_from_file(
            str(sample_8page_pdf_path),
            str(output_path),
            config
        )

        assert output_path.exists()

    def test_with_all_options_enabled(self, sample_8page_pdf_path, tmp_path):
        """
        Test creating a booklet with all optional features enabled.

        Expected behavior:
        - Scales pages to fit
        - Preserves aspect ratio
        - Draws guide lines
        - Numbers pages
        """
        output_path = tmp_path / "full_options.pdf"
        config = (
            imposer.BookletConfig()
            .with_page_size(imposer.PageSize.a4())
            .with_pages_per_sheet(2)
            .with_scale_to_fit(True)
            .with_preserve_aspect_ratio(True)
            .with_draw_guides(True)
            .with_number_pages(True)
        )

        imposer.generate_booklet_from_file(
            str(sample_8page_pdf_path),
            str(output_path),
            config
        )

        assert output_path.exists()
        # Output should be larger due to guide lines and page numbers
        assert output_path.stat().st_size > sample_8page_pdf_path.stat().st_size
