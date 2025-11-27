#!/usr/bin/env python3
"""
Example script demonstrating the imposer Python bindings.

This script shows how to use the imposer library to create various types
of booklet impositions from PDF files.
"""

import imposer
import sys
from pathlib import Path


def example_basic_booklet(input_path: str, output_path: str):
    """
    Create a basic A4 2-up saddle-stitch booklet.

    This is the most common booklet format:
    - A4 page size
    - 2 pages per sheet (1 on each side)
    - Saddle-stitch binding (stapled in the middle)
    """
    print("Creating basic A4 2-up booklet...")

    config = imposer.BookletConfig()
    imposer.generate_booklet_from_file(input_path, output_path, config)

    print(f"✓ Created: {output_path}")


def example_letter_booklet(input_path: str, output_path: str):
    """Create a Letter-sized booklet (US paper size)."""
    print("Creating Letter-sized booklet...")

    config = (
        imposer.BookletConfig()
        .with_page_size(imposer.PageSize.letter())
    )
    imposer.generate_booklet_from_file(input_path, output_path, config)

    print(f"✓ Created: {output_path}")


def example_3up_booklet(input_path: str, output_path: str):
    """
    Create a 3-up booklet (3 pages per sheet).

    Useful for creating tri-fold brochures or fitting more content
    on each sheet.
    """
    print("Creating 3-up booklet...")

    config = (
        imposer.BookletConfig()
        .with_pages_per_sheet(3)
        .with_draw_guides(True)  # Draw cut/fold lines
    )
    imposer.generate_booklet_from_file(input_path, output_path, config)

    print(f"✓ Created: {output_path}")


def example_perfect_bound_booklet(input_path: str, output_path: str):
    """
    Create a perfect-bound booklet with signatures.

    Perfect binding is used for thicker books where pages are glued
    together at the spine. The book is divided into signatures
    (groups of folded sheets).
    """
    print("Creating perfect-bound booklet with signatures...")

    binding = imposer.BindingType.perfect_bound(
        sheets_per_signature=8,  # 8 sheets (16 pages) per signature
        num_signatures=3          # 3 signatures total
    )

    config = (
        imposer.BookletConfig()
        .with_binding_type(binding)
    )
    imposer.generate_booklet_from_file(input_path, output_path, config)

    print(f"✓ Created: {output_path}")


def example_booklet_with_guides(input_path: str, output_path: str):
    """
    Create a booklet with cut/fold guides and page numbers.

    Useful for manual cutting and assembly:
    - Draws guide lines showing where to cut/fold
    - Numbers each page for easy verification
    """
    print("Creating booklet with guides and page numbers...")

    config = (
        imposer.BookletConfig()
        .with_draw_guides(True)
        .with_number_pages(True)
        .with_scale_to_fit(True)
        .with_preserve_aspect_ratio(True)
    )
    imposer.generate_booklet_from_file(input_path, output_path, config)

    print(f"✓ Created: {output_path}")


def example_in_memory_processing(input_path: str, output_path: str):
    """
    Process a PDF in memory without writing intermediate files.

    Useful when integrating with other Python libraries or when
    you want to process the PDF data directly.
    """
    print("Processing PDF in memory...")

    # Read the input PDF
    with open(input_path, 'rb') as f:
        input_bytes = f.read()

    # Configure and generate booklet
    config = (
        imposer.BookletConfig()
        .with_page_size(imposer.PageSize.a4())
        .with_pages_per_sheet(2)
    )

    # Process in memory
    output_bytes = imposer.generate_booklet(input_bytes, config)

    # Write the result
    with open(output_path, 'wb') as f:
        f.write(output_bytes)

    print(f"✓ Created: {output_path}")


def example_scribus_integration(input_path: str, output_path: str):
    """
    Example showing how to use imposer from a Scribus Python script.

    This demonstrates the pattern for integrating imposer into Scribus
    automation workflows.
    """
    print("Scribus integration example...")

    # In a real Scribus script, you would:
    # 1. Export the Scribus document to PDF using scribus.pdfExport()
    # 2. Process the PDF with imposer
    # 3. Optionally re-import or display the result

    config = (
        imposer.BookletConfig()
        .with_page_size(imposer.PageSize.a4())
        .with_draw_guides(True)
    )

    imposer.generate_booklet_from_file(input_path, output_path, config)

    print(f"✓ Created: {output_path}")
    print("\nTo use in Scribus:")
    print("1. Copy this script to ~/.scribus/scripts/")
    print("2. In Scribus: Script > Execute Script")
    print("3. Select this script and run it")


def main():
    """Run all examples if an input PDF is provided."""
    if len(sys.argv) < 2:
        print("Usage: python example.py <input.pdf>")
        print("\nThis script demonstrates various imposer features by creating")
        print("different types of booklet impositions from your PDF.")
        sys.exit(1)

    input_path = sys.argv[1]

    if not Path(input_path).exists():
        print(f"Error: File not found: {input_path}")
        sys.exit(1)

    # Create output directory
    output_dir = Path("examples_output")
    output_dir.mkdir(exist_ok=True)

    # Get input filename without extension
    input_name = Path(input_path).stem

    print(f"\nProcessing: {input_path}\n")
    print("=" * 60)

    try:
        # Run all examples
        example_basic_booklet(
            input_path,
            str(output_dir / f"{input_name}_basic.pdf")
        )

        example_letter_booklet(
            input_path,
            str(output_dir / f"{input_name}_letter.pdf")
        )

        example_3up_booklet(
            input_path,
            str(output_dir / f"{input_name}_3up.pdf")
        )

        example_perfect_bound_booklet(
            input_path,
            str(output_dir / f"{input_name}_perfect_bound.pdf")
        )

        example_booklet_with_guides(
            input_path,
            str(output_dir / f"{input_name}_with_guides.pdf")
        )

        example_in_memory_processing(
            input_path,
            str(output_dir / f"{input_name}_in_memory.pdf")
        )

        example_scribus_integration(
            input_path,
            str(output_dir / f"{input_name}_scribus.pdf")
        )

        print("\n" + "=" * 60)
        print(f"\n✓ All examples completed successfully!")
        print(f"  Output files are in: {output_dir}/")

    except Exception as e:
        print(f"\n✗ Error: {e}")
        sys.exit(1)


if __name__ == "__main__":
    main()
