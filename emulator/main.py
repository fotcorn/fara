import sys
import threading

from .machine_state import MachineState
from .loop import run

import pygame
import pygame.ftfont

SCREEN_WIDTH = 100
SCREEN_HEIGHT = 30
SCREEN_OFFSET = 0x10000


def main():
    if len(sys.argv) != 2:
        print(f'{sys.argv[0]} <file.bin>')
        sys.exit(1)
    with open(sys.argv[1], 'rb') as f:
        code = f.read()

    state = MachineState()
    state.memory[0x1000:0x1000+len(code)] = code
    state.pc = 0x1000

    state.memory[SCREEN_OFFSET:SCREEN_OFFSET+SCREEN_WIDTH*SCREEN_HEIGHT] = [0] * (SCREEN_WIDTH * SCREEN_HEIGHT)

    pygame.init()
    pygame.ftfont.init()

    screen = pygame.display.set_mode((1280, 720))
    font_name = pygame.font.match_font('dejavu sans mono')
    font = pygame.ftfont.Font(font_name, 170)

    background = pygame.Surface(screen.get_size())
    background.fill((0, 0, 0))
    background = background.convert()

    emu_thread = threading.Thread(target=run, args=(state,), daemon=True)
    emu_thread.start()

    while state.running:
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                state.running = False
            elif event.type == pygame.KEYDOWN and event.unicode != '':
                state.stdin.append(ord(event.unicode))

        screen.blit(background, (0, 0))

        display = state.memory[SCREEN_OFFSET:SCREEN_OFFSET+SCREEN_WIDTH*SCREEN_HEIGHT]
        display = ''.join([chr(c) if 32 < c < 127 else '' for c in display])

        for i in range(SCREEN_HEIGHT):
            line = display[i*SCREEN_WIDTH:(i+1)*SCREEN_WIDTH]
            message = font.render(line, True, (255, 255, 255))
            rect = message.get_rect()
            surface = pygame.transform.smoothscale(message, (rect.width // 8, rect.height // 8))
            screen.blit(surface, (0, 3 + i * 23))

        pygame.display.flip()

    pygame.quit()


if __name__ == '__main__':
    main()
