#include <stdio.h>
#include <stdlib.h>
#include <string.h>

const double pi = 3.14;

int own_strlen(const char *s)
{
    // count until \0 is encountered
    int i = 0;
    while (  s[i] != '\0' )
    {
        i++;
    }
    return i;
}

int modify_str(char *s)
{
    s[0] = '\0';
    return 0;
}

int main(int argc, char *argv[])
{
    if ( argc != 2)
    {
        printf("Usage: %s <string-enclosed-in-quotes>\n", argv[0]);
        return 0;
    }

    // Some string length counting
    const int input_length = own_strlen(argv[1]);
    printf("String length is: %d\n", input_length);

    // copying the user input string into my own buffer
    char *destination_buffer = malloc(input_length);
    strcpy(destination_buffer, argv[1]);

    printf("Copied string is: %s\n", destination_buffer);
    free(destination_buffer);
}
