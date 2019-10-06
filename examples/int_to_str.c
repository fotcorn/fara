#include <stdio.h>
#include <stdlib.h>

char* int_to_char(int x) {
    int i = 1;
    for (int j = 10; j <= 1000000000; i++, j = j * 10) {
        if (x % j == x) {
            break;
        }
    }

    char* string = malloc((i + 1) * sizeof(char));

    for (int k = 0; k < i; k++) {
        int value = x % 10;
        string[i - k - 1] = (char)value + 48;
        x = x / 10;
    }

    string[i] = '\0';
    return string;
}

int main(void) {
    printf("%s\n", int_to_char(123123));
    return 0;
}
