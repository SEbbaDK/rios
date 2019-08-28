import java.io.File
import java.lang.RuntimeException

open class CodeCoordinate(
    val location: Int,
    val line: Int,
    val column: Int
)
class Token(
    coordinate: CodeCoordinate,
    val type : TokenType,
    val value : Any?,
    val code : String
) : CodeCoordinate(coordinate.location, coordinate.line, coordinate.column)
enum class TokenType(val code: String?)
{
    StateId,
    NameId,

    RWhen    ("when"),
    RAlways  ("always"),
    REvery   ("every"),
    RAfter   ("after"),
    ROnenter ("onenter"),

    UDay    ("d"),
    UHour   ("h"),
    UMinute ("m"),
    USecond ("s"),
    UMillisecond ("ms"),
    UMicrosecond ("µs"),

    OAdd      ("+"),
    OSubtract ("-"),
    OMultiply ("*"),
    ODivide   ("/"),
    OModulo   ("%"),

    OBoolAnd ("&&"),
    OBoolOr  ("||"),
    OBoolXor ("^^"),
    OBoolNeg ("!"),

    OBitAnd     ("&"),
    OBitOr      ("|"),
    OBitXor     ("^"),
    OBitNeg     ("~"),
    OShiftLeft  ("<<"),
    OShiftRight (">>"),

    OGreat       (">"),
    OGreatEquals (">="),
    OLess        ("<"),
    OLessEquals  ("<="),
    OEquals      ("=="),
    ONotEquals   ("!="),
    OChange      ("->"),
    ONotChange   ("!>"),

    OOld   ("§"),
    ODeref ("*"),

    TLong   ("long"),
    TInt    ("int"),
    TShort  ("short"),
    TBool   ("bool"),
    TString ("string"),
    TChar   ("char"),
    TProc   ("proc"),
    TFunc   ("func"),

    CBit,
    CHex,
    CDec,
    CBool,
    CString,
    CChar,
    CPin,
    CSerial;
    constructor() : this(null)
    companion object {
        val hashes = TokenType.values().map{ t -> t.code.hashCode() }.toSet()
        fun contains(input : String) = hashes.contains(input.hashCode())
        fun get(input : String) =
            values().filter{ t -> t.code == input }.firstOrNull()
                ?: throw RuntimeException("Attempted to get a TokenType with an invalid code-bit")
    }
}

class Lexer(file: String)
{
    private val input = File(file).readText()
    private var location = 0
    private var line = 0
    private var column = 0

    private fun advance(length : Int)
    {
        for (i in 0..length)
        {
            when (peek())
            {
                '\n' -> { line++ ; column = 1 }
                else -> { column++ }
            }
            location++
        }
    }

    private fun peek() : Char = when
    {
        location > input.length -> '\u0000'
        else                    -> input[location]
    }
    private fun peek(string : String) : Boolean = when
    {
        location > input.length -> false
        location + string.length > input.length -> false
        else -> input.substring(location, location + string.length) == string
    }

    private fun ignore()
    {
        while (peek().isWhitespace() || peek() in setOf('/', '#')) {
            when (peek()) {
                '#'  -> ignoreUntil("\n")
                '/' -> when {
                    peek("//") -> ignoreUntil("\n")
                    peek("/*") -> ignoreUntil("*/")
                }
                else -> { advance(1) }
            }
        }
    }

    private fun ignoreUntil(limit : String)
    {
        while (!peek(limit))
            advance(1)
        advance(limit.length)
    }

    fun lex() : Token = when
    {
        peek.isWhitespace() -> { ignore() ; lex() }
        peek.isLetter() -> lexNameId()
    }

    private fun lexNameId() : Token
    {
        val coordinate = CodeCoordinate(location, line, column)
        val start = location
        while (peek().isLetter())
            advance(1)
        val end = location
        val name = input.substring(start, end)
        if(TokenType.contains(name))
            return Token(coordinate, TokenType.get(name), name, name)
        return Token(
            coordinate,
            if (name[0].isUpperCase()) TokenType.StateId else TokenType.NameId,
            name,
            name
        )
    }
}