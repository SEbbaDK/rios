enum tokentype_enum
{
    NameId,
    StateId,

    Reaction,
    Type,
    Unit,

    //Operators
    BoolOp,
    CompOp,
    AritAddOp,
    AritMultOp,
};
typedef enum tokentype_enum tokentype;

struct token_struct {
    int location, line, column;
    tokentype type;
};
typedef struct token_struct token;
