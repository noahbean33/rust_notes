#include <stdio.h>

int encryption_key = 42;

int main()
{
    // char -> 8 bit number (values 0..255)
    char c;
    while ( (c = getc(stdin)) != EOF )
    {
        // write ANY transformation here -> converts ANY input to ANY output
        putc(c ^ encryption_key, stdout);
    }
}
