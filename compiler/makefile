.PHONY: run clean

run: clean rios
	./rios

clean:
	rm -f rios

rios:
	gcc lexer.c compiler.c -o rios
