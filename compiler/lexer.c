#include <stdio.h>
#include "token.h"

char buffer[1024];
FILE *file;
int location, line, column;

void openFile(const char* filename)
{
    file = fopen("test", "r");
    if(!file)
        printf("Couldn't find/open file\n");
}

void closeFile()
{
    fclose(file);
}

token lex()
{
    char c;
    if((c = fgetc(file)) != EOF)
        printf("%c\n", c);
    token scannedToken;
    return(scannedToken);
}
