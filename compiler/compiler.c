#include <stdio.h>
#include "lexer.h"

int main()
{
    printf("Hello World\n");
    openFile("test");
    lex();
    closeFile();
    return 0;
}
