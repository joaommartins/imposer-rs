import imposer


def test_perfect_bound_accepts_kwargs():
    # Should accept keyword args and return a BindingType
    b = imposer.BindingType.perfect_bound(sheets_per_signature=4, num_signatures=1)
    assert b is not None


def test_bookletconfig_chain_with_binding_and_sheets():
    # Create binding and then set it on BookletConfig
    binding = imposer.BindingType.perfect_bound(sheets_per_signature=2, num_signatures=1)
    cfg = imposer.BookletConfig().with_binding_type(binding).with_sheets_per_signature(2)
    assert cfg is not None
