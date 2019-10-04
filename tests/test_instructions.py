from tests.utils import execute


def test_mov():
    state = execute('''
    mov 10, %i0
    ''')
    assert state.i0 == 10


def test_ld_str():
    state = execute('''
    mov 10, %i0
    str1 %i0, $100
    ld1 $100, %i1
    ''')
    assert state.i1 == 10


# arithmetic
def test_add():
    state = execute('''
    mov 5, %i0
    mov 7, %i1
    add %i1, %i0
    ''')
    assert state.i0 == 12


def test_sub():
    state = execute('''
    mov 5, %i0
    mov 7, %i1
    sub %i1, %i0
    ''')
    assert state.i0 == 2


def test_inc():
    state = execute('''
    mov 5, %i0
    inc %i0
    ''')
    assert state.i0 == 6


def test_dec():
    state = execute('''
    mov 5, %i0
    dec %i0
    ''')
    assert state.i0 == 4


def test_mul():
    state = execute('''
    mov 5, %i0
    mov 7, %i1
    mul %i1, %i0
    ''')
    assert state.i0 == 35


def test_div():
    state = execute('''
    mov 15, %i1
    mov 5, %i0
    div %i1, %i0
    ''')
    assert state.i0 == 3


def test_mod():
    state = execute('''
    mov 7, %i1
    mov 5, %i0
    mod %i1, %i0
    ''')
    assert state.i0 == 2


# binary
def test_and():
    state = execute('''
    mov 7, %i1
    mov 9, %i0
    and %i1, %i0
    ''')
    assert state.i0 == 1


def test_or():
    state = execute('''
    mov 5, %i1
    mov 3, %i0
    or %i1, %i0
    ''')
    assert state.i0 == 7


def test_xor():
    state = execute('''
    mov 5, %i1
    mov 3, %i0
    xor %i1, %i0
    ''')
    assert state.i0 == 6

#
# def test_not():
#     state = execute('''
#     ''')
#     assert state.i0 == 10
#
#
# # compare & jumps
# def test_jmp():
#     state = execute('''
#     ''')
#     assert state.i0 == 10
#
#
# def test_je():
#     state = execute('''
#     ''')
#     assert state.i0 == 10
#
#
# def test_jne():
#     state = execute('''
#     ''')
#     assert state.i0 == 10
#
#
# # conditional jumps
# def test_jl():
#     state = execute('''
#     ''')
#     assert state.i0 == 10
#
#
# def test_jle():
#     state = execute('''
#     ''')
#     assert state.i0 == 10
#
#
# def test_jg():
#     state = execute('''
#     ''')
#     assert state.i0 == 10
#
#
# def test_jge():
#     state = execute('''
#     ''')
#     assert state.i0 == 10
#
#
# # stack
# def test_push():
#     state = execute('''
#     ''')
#     assert state.i0 == 10
#
#
# def test_pop():
#     state = execute('''
#     ''')
#     assert state.i0 == 10
#
#
# def test_call():
#     state = execute('''
#     ''')
#     assert state.i0 == 10
#
#
# def test_ret():
#     state = execute('''
#     ''')
#     assert state.i0 == 10


