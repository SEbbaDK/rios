abstract class CompilerException(phase: String, message: String) :
    Throwable("[$phase] : $message")

abstract class LocalCompilerException(phase: String, line: Int, column: Int, message: String) :
    CompilerException(phase, "<line:$line,column:$column> $message")

class LexerException(line: Int, column: Int, message: String) : LocalCompilerException("Lexer", line, column, message)