require "pars3k"

require "./ast.cr"

module RiosParser extend self
    include Pars3k
    def parse(s : String)
        w = Parse.many_of(Parse.char(' ') | Parse.char('\t') | Parse.char('\n') | Parse.char('\r'))
        
        var_name = w >> do_parse({
        	head <= Parse.char('_') | Parse.alphabet_lower,
        	tail <= (Parse.join Parse.many_of Parse.alphabet),
        	Parse.constant head + tail
    	})
        assign = w >> Parse.char '='
        constant = w >> Parse.int

        state_key = w >> Parse.string "state"
        default_key = w >> Parse.string "default"
        state_name = w >> do_parse({
            first <= Parse.alphabet_upper,
            tail <= Parse.join(Parse.many_of(Parse.alphabet())),
            Parse.constant first + tail
        })

        definition = w >> do_parse({
            name <= var_name,
            _ <= assign,
            val <= constant,
            Parse.constant({:Definition, name, val})
        })
        definitions = w >> Parse.char('{') >> Parse.many_of definition << w << Parse.char('}')

        non_default_state = state_key >> do_parse({
            name <= state_name,
            defs <= definitions,
            Parse.constant({:State, name, false, defs})
        })

        default_state = default_key >> state_key >> do_parse({
            name <= state_name,
            defs <= definitions,
            Parse.constant({:State, name, true, defs})
        })
        
        state = w >> (non_default_state | default_state)

        parser = Parse.many_of state
    
        parser.parse s
    end
end
