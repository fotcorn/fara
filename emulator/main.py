import sys
import threading

from .machine_state import MachineState
from .loop import run

import pygame
import pygame.ftfont


def main():
    if len(sys.argv) != 2:
        print(f'{sys.argv[0]} <file.bin>')
        sys.exit(1)
    with open(sys.argv[1], 'rb') as f:
        code = f.read()

    state = MachineState()
    state.memory[1000:1000+len(code)] = code
    state.pc = 1000

    pygame.init()
    pygame.ftfont.init()

    screen = pygame.display.set_mode((1280, 720))
    font_name = pygame.font.match_font('dejavu sans mono')
    font = pygame.ftfont.Font(font_name, 170)

    emu_thread = threading.Thread(target=run, args=(state,), daemon=True)
    emu_thread.start()

    while state.running:
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                running = False
            elif event.type == pygame.KEYDOWN and event.unicode != '':
                state.stdin.append(ord(event.unicode))

        message = font.render("0123456789"*10, True, (255, 255, 255))
        rect = message.get_rect()
        surface = pygame.transform.smoothscale(message, (rect.width//8, rect.height//8))
        screen.blit(surface, (0, 3))

        for i in range(1, 60):
            message = font.render(str(i), True, (255, 255, 255))
            rect = message.get_rect()
            surface = pygame.transform.smoothscale(message, (rect.width // 8, rect.height // 8))
            screen.blit(surface, (0, 3 + i * 23))

        pygame.display.flip()

    pygame.quit()


if __name__ == '__main__':
    main()
