#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define START_BYTES 3

// Reads a full sentence from user input (stdin) and outputs that sentence again.
int main()
{
    unsigned int memory_size = START_BYTES;
    char *text = malloc(memory_size);

    char *sample_corruption = malloc(50);
    memset(text, 0, START_BYTES);

    int c;
    int i = 0;
    while ( (c = getchar()) != EOF)
    {
        if ( c == '\n')
        {
            text[i] = '\0';
            break;
        }

        if (i >= memory_size )
        {
            // Increase (reallocate) the reserved memory
            memory_size = memory_size * 2; // double possible input size
            text = realloc(text, memory_size);
            printf("Resized memory to size: %u bytes\n", memory_size);
        }
        text[i] = c;
        i++;
    }

    strcpy(sample_corruption, "IF THIS STRING IS PRINTED, MEMORY IS CORRUPTED");

    printf("You typed %s\n", text);
}
