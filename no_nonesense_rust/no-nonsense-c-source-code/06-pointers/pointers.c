#include <stdio.h>

// this doesn't work, because number is passed by value
int square_broken(int number)
{
    number = number * number;                          
    printf("Inside the broken function: Number = %d\n", number);
    return 0;
}

// this works, because the number is passed by REFERENCE
int square_working(int *number)
{
    int actual_value = *number;
    *number = actual_value * actual_value;
    printf("Inside the working function: Number = %d\n", *number);
    return 0;
}

int main()
{
    int number = 6;    
    printf("Trying to square the number %d\n", number);
   
    square_broken(number);
    printf("After passing by value: Number = %d\n", number);

    square_working(&number);
    printf("After passing by reference: Number = %d\n", number);
}
