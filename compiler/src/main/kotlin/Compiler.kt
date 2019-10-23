fun main(args: Array<String>)
{
    println("Compiling")
    val lexer = Lexer("test")

    var tokArray = ArrayList<Token>()
    var tok = lexer.lex()
    while (tok.type != TokenType.EOF)
    {
        println("Found token: ${tok}")
        tokArray.add(tok)
        tok = lexer.lex()
    }
    println(tokArray)
}