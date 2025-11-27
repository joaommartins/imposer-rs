from typing import Optional


class PageSize:
    """Page size factory helpers (A4, A3, A5, Letter, Legal, Tabloid)."""

    @staticmethod
    def a4() -> "PageSize": ...

    @staticmethod
    def a3() -> "PageSize": ...

    @staticmethod
    def a5() -> "PageSize": ...

    @staticmethod
    def letter() -> "PageSize": ...

    @staticmethod
    def legal() -> "PageSize": ...

    @staticmethod
    def tabloid() -> "PageSize": ...


class BindingType:
    """Binding type factories."""

    @staticmethod
    def saddle_stitch() -> "BindingType": ...

    @staticmethod
    def perfect_bound(sheets_per_signature: Optional[int] = None, num_signatures: Optional[int] = None) -> "BindingType": ...


class BookletConfig:
    """Builder-style configuration object for booklet generation.

    The class is constructed with no arguments and exposes chainable
    `with_*` methods that return the same object for chaining.
    """

    def __init__(self) -> None: ...

    def with_page_size(self, page_size: PageSize) -> "BookletConfig": ...

    def with_pages_per_sheet(self, pages: int) -> "BookletConfig": ...

    def with_scale_to_fit(self, scale: bool) -> "BookletConfig": ...

    def with_preserve_aspect_ratio(self, preserve: bool) -> "BookletConfig": ...

    def with_draw_guides(self, draw: bool) -> "BookletConfig": ...

    def with_number_pages(self, number: bool) -> "BookletConfig": ...

    def with_binding_type(self, binding_type: BindingType) -> "BookletConfig": ...

    def with_sheets_per_signature(self, sheets: int) -> "BookletConfig": ...

    def with_num_signatures(self, num: Optional[int] = None) -> "BookletConfig": ...


def generate_booklet(pdf_bytes: bytes, config: BookletConfig) -> bytes: ...


def generate_booklet_from_file(input_path: str, output_path: str, config: BookletConfig) -> None: ...
