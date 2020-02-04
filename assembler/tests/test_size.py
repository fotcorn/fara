from tests.utils import execute


def test_add():
    state = execute('''
    mov 5, %i0
    add1u 5, %i0
    ''')
    assert state.i0 == 10


def test_add_unsigned_overflow():
    state = execute('''
    mov 255, %i0
    add1u 100, %i0
    ''')
    assert state.i0 == 99
