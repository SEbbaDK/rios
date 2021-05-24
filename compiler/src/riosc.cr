require "option_parser"

require "./parser.cr"

files = [] of String

OptionParser.parse do |parser|
    parser.banner = "rios compiler"
    
    parser.on "-v", "--version", "Show version" do
        puts "0.1.0"
        exit
    end
    
    parser.on "-h", "--help", "Show help" do
        puts parser
        exit
    end
    
    parser.missing_option do |flag|
        STDERR.puts "ERROR: #{flag} is missing argument(s)"
        STDERR.puts ""
        STDERR.puts parser
        exit 1
    end
    
    parser.invalid_option do |flag|
        STDERR.puts "ERROR: #{flag} is not a known option"
        STDERR.puts ""
        STDERR.puts parser
        exit 1
    end
    
    parser.unknown_args do |args|
        files = args
    end
end

if files.empty?
    puts "No files given for compilation"
    exit 2
end

code = File.read files[0] if files[0] != "-"
code = STDIN.gets '\0', 100000 if files[0] == "-"

if code.nil?
    STDERR.puts "No code to compile"
    exit 3
end

puts "PARSING:"
puts code
puts

ast = RiosParser.parse code
if ast.is_a? RiosParser::ParseError
	ast.message    
end
puts ast

