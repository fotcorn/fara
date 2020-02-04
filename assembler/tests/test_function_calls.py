from tests.utils import execute


def test_no_parameters():
    state = execute('''
    jmp main

    func:
        mov 5, %i0
        ret
    
    main:
        call func
        add 1, %i0
        halt
    ''')
    assert state.i0 == 6


def test_local_variables():
    state = execute('''
    jmp main

    func:
        push %i1
        mov 0, %i1
        add %i0, %i1
        add %i0, %i1
        mov %i1, %i0
        pop %i1
        ret
    
    main:
        mov 3, %i0
        mov 5, %i1
        call func
    ''')
    assert state.i0 == 6
    assert state.i1 == 5
