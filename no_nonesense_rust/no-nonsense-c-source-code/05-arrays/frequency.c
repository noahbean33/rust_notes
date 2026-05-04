#include <stdio.h>
#include <string.h>

#define COUNT 256

// Requires a character_counts array of size 256 (all possible bytes / chars)
void printAllCharacterCounts(int *character_counts)
{
    int frequency;
    for ( int i=32 ; i<126 ; i++) // filter out ASCII characters
    {
        frequency = character_counts[i];
        if ( frequency != 0)
            printf("%c -> %d\n", i, frequency);
    }
}

// A simple character frequency analyzer
int main()
{
    int character_counts[COUNT];
    //character_counts -> memory location
    //character_counts[0] -> value at the first "slot" in this memory location
    //character_counts[59] -> value at the 60th "slot"
    //for (int i=0; i<256; i++) // manually setting all values to 0
    //   character_counts[i] = 0;
    memset(character_counts, 0, COUNT*sizeof(character_counts[0]));


    int c;
    while ( (c=getc(stdin)) != EOF )
    {
        // do something with the character
        character_counts[c] += 1;
        // complete the code for all letters of the alphabet
    } 

    printAllCharacterCounts(character_counts);
}
