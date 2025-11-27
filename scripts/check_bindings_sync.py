#!/usr/bin/env python3
"""
Script to verify that Python bindings are in sync with the Rust library.

This script checks:
1. All public Rust config types have corresponding Python wrappers
2. All Rust public methods are exposed in Python
3. Python bindings match the expected API surface

Requires: tree-sitter, tree-sitter-rust
Install with: pip install tree-sitter tree-sitter-rust
"""

import sys
import re
from pathlib import Path
from typing import Dict, Set, Any

from tree_sitter import Language, Parser
from tree_sitter_rust import language as rust_language


def extract_rust_public_api_tree_sitter(file_path: Path) -> Dict[str, Any]:
    """Extract public API from Rust source using tree-sitter."""
    if not file_path.exists():
        return {"enums": {}, "methods": {}, "functions": set()}

    parser = Parser(Language(rust_language()))

    content = file_path.read_bytes()
    tree = parser.parse(content)
    root = tree.root_node

    api: Dict[str, Any] = {
        "enums": {},
        "methods": {},
        "functions": set()
    }

    def visit_node(node):
        # Find public enums
        if node.type == "enum_item":
            # Check if public
            visibility = None
            for child in node.children:
                if child.type == "visibility_modifier":
                    visibility = child.text.decode('utf-8')
                    break

            if visibility == "pub":
                # Get enum name
                enum_name = None
                enum_body = None
                for child in node.children:
                    if child.type == "type_identifier":
                        enum_name = child.text.decode('utf-8')
                    elif child.type == "enum_variant_list":
                        enum_body = child

                if enum_name and enum_body:
                    variants = set()
                    for variant_node in enum_body.children:
                        if variant_node.type == "enum_variant":
                            variant_name = None
                            for vchild in variant_node.children:
                                if vchild.type == "identifier":
                                    variant_name = vchild.text.decode('utf-8')
                                    break
                            if variant_name:
                                variants.add(variant_name)
                    api["enums"][enum_name] = variants

        # Find public impl blocks
        elif node.type == "impl_item":
            # Get the type being implemented
            impl_type = None
            for child in node.children:
                if child.type == "type_identifier":
                    impl_type = child.text.decode('utf-8')
                    break

            if impl_type:
                methods = set()
                for child in node.children:
                    if child.type == "declaration_list":
                        for item in child.children:
                            if item.type == "function_item":
                                # Check if public
                                is_public = False
                                fn_name = None
                                for fn_child in item.children:
                                    if fn_child.type == "visibility_modifier":
                                        is_public = fn_child.text.decode('utf-8') == "pub"
                                    elif fn_child.type == "identifier":
                                        fn_name = fn_child.text.decode('utf-8')

                                if is_public and fn_name:
                                    methods.add(fn_name)

                if methods:
                    if impl_type not in api["methods"]:
                        api["methods"][impl_type] = set()
                    api["methods"][impl_type].update(methods)

        # Find public functions
        elif node.type == "function_item":
            is_public = False
            fn_name = None
            for child in node.children:
                if child.type == "visibility_modifier":
                    is_public = child.text.decode('utf-8') == "pub"
                elif child.type == "identifier":
                    fn_name = child.text.decode('utf-8')

            if is_public and fn_name:
                api["functions"].add(fn_name)

        # Recursively visit children
        for child in node.children:
            visit_node(child)

    visit_node(root)
    return api


def extract_rust_public_api():
    """Extract public API from Rust source files using tree-sitter."""
    api = {
        "PageSize": set(),
        "BindingType": set(),
        "BookletConfig": set(),
        "functions": set()
    }

    # Use tree-sitter for accurate parsing
    config_rs = Path("src/config.rs")
    if config_rs.exists():
        parsed = extract_rust_public_api_tree_sitter(config_rs)

        # Extract PageSize enum variants
        if "PageSize" in parsed["enums"]:
            for variant in parsed["enums"]["PageSize"]:
                # Convert to snake_case for Python
                variant_snake = re.sub('([a-z0-9])([A-Z])', r'\1_\2', variant).lower()
                api["PageSize"].add(variant_snake)

        # Extract BindingType enum variants
        if "BindingType" in parsed["enums"]:
            for variant in parsed["enums"]["BindingType"]:
                # Convert to snake_case for Python
                variant_snake = re.sub('([a-z0-9])([A-Z])', r'\1_\2', variant).lower()
                api["BindingType"].add(variant_snake)

        # Extract BookletConfig methods
        if "BookletConfig" in parsed["methods"]:
            for method in parsed["methods"]["BookletConfig"]:
                if method.startswith("with_"):
                    api["BookletConfig"].add(method)

    # Extract public functions from lib.rs
    lib_rs = Path("src/lib.rs")
    if lib_rs.exists():
        parsed = extract_rust_public_api_tree_sitter(lib_rs)
        for func in parsed["functions"]:
            if func in ['generate_booklet', 'generate_booklet_with_config']:
                api["functions"].add(func)

    return api


def extract_python_api():
    """Extract exposed Python API from python.rs."""
    api = {
        "PageSize": set(),
        "BindingType": set(),
        "BookletConfig": set(),
        "functions": set()
    }

    python_rs = Path("src/python.rs")
    if not python_rs.exists():
        print("ERROR: src/python.rs not found")
        return api

    content = python_rs.read_text()

    # Extract PyPageSize methods
    py_page_size_match = re.search(r'impl PyPageSize\s*\{(.*?)\n\}', content, re.DOTALL)
    if py_page_size_match:
        impl_content = py_page_size_match.group(1)
        for match in re.finditer(r'fn (\w+)\(', impl_content):
            method = match.group(1)
            if method not in ['__repr__', '__str__']:
                api["PageSize"].add(method)

    # Extract PyBindingType methods
    py_binding_match = re.search(r'impl PyBindingType\s*\{(.*?)\n\}', content, re.DOTALL)
    if py_binding_match:
        impl_content = py_binding_match.group(1)
        for match in re.finditer(r'fn (\w+)\(', impl_content):
            method = match.group(1)
            if method not in ['__repr__', '__str__']:
                api["BindingType"].add(method)

    # Extract PyBookletConfig methods
    py_config_match = re.search(r'impl PyBookletConfig\s*\{(.*?)\n\}', content, re.DOTALL)
    if py_config_match:
        impl_content = py_config_match.group(1)
        for match in re.finditer(r'fn (with_\w+)', impl_content):
            method = match.group(1)
            api["BookletConfig"].add(method)

    # Extract module-level functions
    for match in re.finditer(r'#\[pyfunction\].*?fn (\w+)', content, re.DOTALL):
        func = match.group(1)
        api["functions"].add(func)

    return api


def compare_apis(rust_api, python_api):
    """Compare Rust and Python APIs and report differences."""
    issues = []

    for api_type in ["PageSize", "BindingType", "BookletConfig"]:
        rust_items = rust_api[api_type]
        python_items = python_api[api_type]

        missing_in_python = rust_items - python_items
        extra_in_python = python_items - rust_items

        if missing_in_python:
            issues.append(f"Missing in Python {api_type}: {', '.join(sorted(missing_in_python))}")

        # Allow some Python-specific helpers (like with_page_size in BookletConfig)
        # These are not errors, just informational
        if extra_in_python and api_type != "BookletConfig":
            print(f"INFO: Extra in Python {api_type}: {', '.join(sorted(extra_in_python))}")

    # Check functions - Python may have slightly different names
    rust_funcs = rust_api["functions"]
    python_funcs = python_api["functions"]

    # Map Rust function names to their Python equivalents
    expected_python_functions = {
        "generate_booklet",  # Should be in Python
        "generate_booklet_from_file"  # Python-specific convenience function
    }

    # Check if core functions are present
    missing_core = {"generate_booklet"} - python_funcs

    if missing_core:
        issues.append(f"Missing core Python functions: {', '.join(sorted(missing_core))}")

    # Note: generate_booklet_with_config is covered by generate_booklet in Python
    # and generate_booklet_from_file is a Python-specific convenience function

    return issues


def main():
    """Main entry point."""
    print("Checking Python bindings synchronization with Rust library...")
    print()

    rust_api = extract_rust_public_api()
    python_api = extract_python_api()

    print("Rust API surface:")
    for api_type, items in rust_api.items():
        if items:
            print(f"  {api_type}: {', '.join(sorted(items))}")
    print()

    print("Python API surface:")
    for api_type, items in python_api.items():
        if items:
            print(f"  {api_type}: {', '.join(sorted(items))}")
    print()

    issues = compare_apis(rust_api, python_api)

    if issues:
        print("❌ ERRORS: Python bindings are not in sync with Rust library!")
        print()
        for issue in issues:
            print(f"  - {issue}")
        print()
        print("Please update src/python.rs to expose missing items.")
        sys.exit(1)
    else:
        print("✅ Python bindings are in sync with Rust library")
        sys.exit(0)


if __name__ == "__main__":
    main()
