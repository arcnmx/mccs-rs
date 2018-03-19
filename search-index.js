var searchIndex = {};
searchIndex["mccs"] = {"doc":"VESA Monitor Command Control Set standardizes the meaning of DDC/CI VCP feature codes, and allows a display to broadcast its capabilities to the host.","items":[[3,"Value","mccs","VCP Value",null,null],[12,"ty","","Specifies the type of the value, continuous or non-continuous.",0,null],[12,"mh","","The high byte of the maximum allowed value.",0,null],[12,"ml","","The low byte of the maximum allowed value.",0,null],[12,"sh","","The high byte of the value.",0,null],[12,"sl","","The low byte of the value.",0,null],[3,"Capabilities","","Parsed display capabilities string.",null,null],[12,"protocol","","The protocol class.",1,null],[12,"ty","","The type of display.",1,null],[12,"model","","The model name/number of the display.",1,null],[12,"commands","","A list of the supported VCP commands.",1,null],[12,"ms_whql","","A value of `1` seems to indicate that the monitor has passed Microsoft's Windows Hardware Quality Labs testing.",1,null],[12,"mccs_version","","Monitor Command Control Set version code.",1,null],[12,"vcp_features","","Virtual Control Panel feature code descriptors.",1,null],[12,"edid","","Extended Display Identification Data",1,null],[12,"vdif","","Video Display Information Format are optional extension blocks for the EDID. Like the EDID field, this is probably not in use.",1,null],[12,"unknown_tags","","Additional unrecognized data from the capability string.",1,null],[3,"Version","","Monitor Command Control Set specification version code",null,null],[12,"major","","Major version number",2,null],[12,"minor","","Minor revision version",2,null],[3,"VcpDescriptor","","Descriptive information about a supported VCP feature code.",null,null],[12,"name","","The name of the feature code, if different from the standard MCCS spec.",3,null],[12,"values","","Allowed values for this feature, and optionally their names.",3,null],[3,"UnknownTag","","An unrecognized entry in the capability string",null,null],[12,"name","","The name of the entry",4,null],[12,"data","","The data contained in the entry, usually an unparsed string.",4,null],[4,"Void","","An error type that cannot be encountered.",null,null],[4,"Protocol","","Display protocol class",null,null],[13,"Monitor","","Standard monitor",5,null],[13,"Display","","I have never seen this outside of an MCCS spec example, it may be a typo.",5,null],[13,"Unknown","","Unrecognized protocol class",5,null],[4,"Type","","Display type",null,null],[13,"Crt","","Cathode Ray Tube display",6,null],[13,"Lcd","","Liquid Crystal Display",6,null],[13,"Led","","Also an LCD, I'm not sure this should exist.",6,null],[13,"Unknown","","Unrecognized display type",6,null],[4,"UnknownData","","Data that can be contained in a capability entry.",null,null],[13,"String","","UTF-8/ASCII data",7,null],[13,"StringBytes","","Data that is not valid UTF-8",7,null],[13,"Binary","","Length-prefixed binary data",7,null],[6,"FeatureCode","","VCP feature code",null,null],[6,"EdidData","","Extended Display Identification Data",null,null],[6,"VdifData","","Video Display Information Format",null,null],[6,"ValueNames","","VCP feature value names",null,null],[11,"clone","","",0,{"inputs":[{"name":"self"}],"output":{"name":"value"}}],[11,"default","","",0,{"inputs":[],"output":{"name":"value"}}],[11,"eq","","",0,{"inputs":[{"name":"self"},{"name":"value"}],"output":{"name":"bool"}}],[11,"ne","","",0,{"inputs":[{"name":"self"},{"name":"value"}],"output":{"name":"bool"}}],[11,"partial_cmp","","",0,{"inputs":[{"name":"self"},{"name":"value"}],"output":{"generics":["ordering"],"name":"option"}}],[11,"lt","","",0,{"inputs":[{"name":"self"},{"name":"value"}],"output":{"name":"bool"}}],[11,"le","","",0,{"inputs":[{"name":"self"},{"name":"value"}],"output":{"name":"bool"}}],[11,"gt","","",0,{"inputs":[{"name":"self"},{"name":"value"}],"output":{"name":"bool"}}],[11,"ge","","",0,{"inputs":[{"name":"self"},{"name":"value"}],"output":{"name":"bool"}}],[11,"cmp","","",0,{"inputs":[{"name":"self"},{"name":"value"}],"output":{"name":"ordering"}}],[11,"hash","","",0,null],[11,"from_value","","Create a new `Value` from a scalar value.",0,{"inputs":[{"name":"u16"}],"output":{"name":"self"}}],[11,"value","","Combines the value bytes into a single value.",0,{"inputs":[{"name":"self"}],"output":{"name":"u16"}}],[11,"maximum","","Combines the maximum value bytes into a single value.",0,{"inputs":[{"name":"self"}],"output":{"name":"u16"}}],[11,"fmt","","",0,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"fmt","","",1,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"default","","",1,{"inputs":[],"output":{"name":"capabilities"}}],[11,"clone","","",1,{"inputs":[{"name":"self"}],"output":{"name":"capabilities"}}],[11,"eq","","",1,{"inputs":[{"name":"self"},{"name":"capabilities"}],"output":{"name":"bool"}}],[11,"ne","","",1,{"inputs":[{"name":"self"},{"name":"capabilities"}],"output":{"name":"bool"}}],[11,"partial_cmp","","",1,{"inputs":[{"name":"self"},{"name":"capabilities"}],"output":{"generics":["ordering"],"name":"option"}}],[11,"lt","","",1,{"inputs":[{"name":"self"},{"name":"capabilities"}],"output":{"name":"bool"}}],[11,"le","","",1,{"inputs":[{"name":"self"},{"name":"capabilities"}],"output":{"name":"bool"}}],[11,"gt","","",1,{"inputs":[{"name":"self"},{"name":"capabilities"}],"output":{"name":"bool"}}],[11,"ge","","",1,{"inputs":[{"name":"self"},{"name":"capabilities"}],"output":{"name":"bool"}}],[11,"cmp","","",1,{"inputs":[{"name":"self"},{"name":"capabilities"}],"output":{"name":"ordering"}}],[11,"hash","","",1,null],[11,"fmt","","",5,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",5,{"inputs":[{"name":"self"}],"output":{"name":"protocol"}}],[11,"eq","","",5,{"inputs":[{"name":"self"},{"name":"protocol"}],"output":{"name":"bool"}}],[11,"ne","","",5,{"inputs":[{"name":"self"},{"name":"protocol"}],"output":{"name":"bool"}}],[11,"partial_cmp","","",5,{"inputs":[{"name":"self"},{"name":"protocol"}],"output":{"generics":["ordering"],"name":"option"}}],[11,"lt","","",5,{"inputs":[{"name":"self"},{"name":"protocol"}],"output":{"name":"bool"}}],[11,"le","","",5,{"inputs":[{"name":"self"},{"name":"protocol"}],"output":{"name":"bool"}}],[11,"gt","","",5,{"inputs":[{"name":"self"},{"name":"protocol"}],"output":{"name":"bool"}}],[11,"ge","","",5,{"inputs":[{"name":"self"},{"name":"protocol"}],"output":{"name":"bool"}}],[11,"cmp","","",5,{"inputs":[{"name":"self"},{"name":"protocol"}],"output":{"name":"ordering"}}],[11,"hash","","",5,null],[11,"from","","",5,{"inputs":[{"name":"str"}],"output":{"name":"self"}}],[11,"fmt","","",5,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"from_str","","",5,{"inputs":[{"name":"str"}],"output":{"name":"result"}}],[11,"fmt","","",6,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",6,{"inputs":[{"name":"self"}],"output":{"name":"type"}}],[11,"eq","","",6,{"inputs":[{"name":"self"},{"name":"type"}],"output":{"name":"bool"}}],[11,"ne","","",6,{"inputs":[{"name":"self"},{"name":"type"}],"output":{"name":"bool"}}],[11,"partial_cmp","","",6,{"inputs":[{"name":"self"},{"name":"type"}],"output":{"generics":["ordering"],"name":"option"}}],[11,"lt","","",6,{"inputs":[{"name":"self"},{"name":"type"}],"output":{"name":"bool"}}],[11,"le","","",6,{"inputs":[{"name":"self"},{"name":"type"}],"output":{"name":"bool"}}],[11,"gt","","",6,{"inputs":[{"name":"self"},{"name":"type"}],"output":{"name":"bool"}}],[11,"ge","","",6,{"inputs":[{"name":"self"},{"name":"type"}],"output":{"name":"bool"}}],[11,"cmp","","",6,{"inputs":[{"name":"self"},{"name":"type"}],"output":{"name":"ordering"}}],[11,"hash","","",6,null],[11,"from","","",6,{"inputs":[{"name":"str"}],"output":{"name":"self"}}],[11,"fmt","","",6,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"from_str","","",6,{"inputs":[{"name":"str"}],"output":{"name":"result"}}],[11,"fmt","","",2,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"default","","",2,{"inputs":[],"output":{"name":"version"}}],[11,"clone","","",2,{"inputs":[{"name":"self"}],"output":{"name":"version"}}],[11,"eq","","",2,{"inputs":[{"name":"self"},{"name":"version"}],"output":{"name":"bool"}}],[11,"ne","","",2,{"inputs":[{"name":"self"},{"name":"version"}],"output":{"name":"bool"}}],[11,"partial_cmp","","",2,{"inputs":[{"name":"self"},{"name":"version"}],"output":{"generics":["ordering"],"name":"option"}}],[11,"lt","","",2,{"inputs":[{"name":"self"},{"name":"version"}],"output":{"name":"bool"}}],[11,"le","","",2,{"inputs":[{"name":"self"},{"name":"version"}],"output":{"name":"bool"}}],[11,"gt","","",2,{"inputs":[{"name":"self"},{"name":"version"}],"output":{"name":"bool"}}],[11,"ge","","",2,{"inputs":[{"name":"self"},{"name":"version"}],"output":{"name":"bool"}}],[11,"cmp","","",2,{"inputs":[{"name":"self"},{"name":"version"}],"output":{"name":"ordering"}}],[11,"hash","","",2,null],[11,"new","","Create a new MCCS version from the specified version and revision.",2,{"inputs":[{"name":"u8"},{"name":"u8"}],"output":{"name":"self"}}],[11,"fmt","","",3,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"default","","",3,{"inputs":[],"output":{"name":"vcpdescriptor"}}],[11,"clone","","",3,{"inputs":[{"name":"self"}],"output":{"name":"vcpdescriptor"}}],[11,"eq","","",3,{"inputs":[{"name":"self"},{"name":"vcpdescriptor"}],"output":{"name":"bool"}}],[11,"ne","","",3,{"inputs":[{"name":"self"},{"name":"vcpdescriptor"}],"output":{"name":"bool"}}],[11,"partial_cmp","","",3,{"inputs":[{"name":"self"},{"name":"vcpdescriptor"}],"output":{"generics":["ordering"],"name":"option"}}],[11,"lt","","",3,{"inputs":[{"name":"self"},{"name":"vcpdescriptor"}],"output":{"name":"bool"}}],[11,"le","","",3,{"inputs":[{"name":"self"},{"name":"vcpdescriptor"}],"output":{"name":"bool"}}],[11,"gt","","",3,{"inputs":[{"name":"self"},{"name":"vcpdescriptor"}],"output":{"name":"bool"}}],[11,"ge","","",3,{"inputs":[{"name":"self"},{"name":"vcpdescriptor"}],"output":{"name":"bool"}}],[11,"cmp","","",3,{"inputs":[{"name":"self"},{"name":"vcpdescriptor"}],"output":{"name":"ordering"}}],[11,"hash","","",3,null],[11,"values","","The allowed values for this feature code.",3,{"inputs":[{"name":"self"}],"output":{"generics":["u8","option"],"name":"keys"}}],[11,"fmt","","",4,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",4,{"inputs":[{"name":"self"}],"output":{"name":"unknowntag"}}],[11,"eq","","",4,{"inputs":[{"name":"self"},{"name":"unknowntag"}],"output":{"name":"bool"}}],[11,"ne","","",4,{"inputs":[{"name":"self"},{"name":"unknowntag"}],"output":{"name":"bool"}}],[11,"partial_cmp","","",4,{"inputs":[{"name":"self"},{"name":"unknowntag"}],"output":{"generics":["ordering"],"name":"option"}}],[11,"lt","","",4,{"inputs":[{"name":"self"},{"name":"unknowntag"}],"output":{"name":"bool"}}],[11,"le","","",4,{"inputs":[{"name":"self"},{"name":"unknowntag"}],"output":{"name":"bool"}}],[11,"gt","","",4,{"inputs":[{"name":"self"},{"name":"unknowntag"}],"output":{"name":"bool"}}],[11,"ge","","",4,{"inputs":[{"name":"self"},{"name":"unknowntag"}],"output":{"name":"bool"}}],[11,"cmp","","",4,{"inputs":[{"name":"self"},{"name":"unknowntag"}],"output":{"name":"ordering"}}],[11,"hash","","",4,null],[11,"fmt","","",7,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",7,{"inputs":[{"name":"self"}],"output":{"name":"unknowndata"}}],[11,"eq","","",7,{"inputs":[{"name":"self"},{"name":"unknowndata"}],"output":{"name":"bool"}}],[11,"ne","","",7,{"inputs":[{"name":"self"},{"name":"unknowndata"}],"output":{"name":"bool"}}],[11,"partial_cmp","","",7,{"inputs":[{"name":"self"},{"name":"unknowndata"}],"output":{"generics":["ordering"],"name":"option"}}],[11,"lt","","",7,{"inputs":[{"name":"self"},{"name":"unknowndata"}],"output":{"name":"bool"}}],[11,"le","","",7,{"inputs":[{"name":"self"},{"name":"unknowndata"}],"output":{"name":"bool"}}],[11,"gt","","",7,{"inputs":[{"name":"self"},{"name":"unknowndata"}],"output":{"name":"bool"}}],[11,"ge","","",7,{"inputs":[{"name":"self"},{"name":"unknowndata"}],"output":{"name":"bool"}}],[11,"cmp","","",7,{"inputs":[{"name":"self"},{"name":"unknowndata"}],"output":{"name":"ordering"}}],[11,"hash","","",7,null]],"paths":[[3,"Value"],[3,"Capabilities"],[3,"Version"],[3,"VcpDescriptor"],[3,"UnknownTag"],[4,"Protocol"],[4,"Type"],[4,"UnknownData"]]};
searchIndex["mccs_caps"] = {"doc":"MCCS compliant displays will report their supported capabilities in a string retrieved over DDC/CI. The format of this string is specified in the DDC specification, MCCS, and ACCESS.bus section 7. This crate parses the capability string into structured data.","items":[[5,"parse_capabilities","mccs_caps","Parses a MCCS capability string.",null,{"inputs":[{"name":"c"}],"output":{"generics":["capabilities"],"name":"result"}}],[14,"dbg_dmp","","Prints a message and the input if the parser fails",null,null],[14,"i32","","if the parameter is nom::Endianness::Big, parse a big endian i32 integer, otherwise a little endian i32 integer",null,null],[14,"preceded","","`preceded!(I -> IResult<I,T>, I -> IResult<I,O>) => I -> IResult<I, O>` preceded(opening, X) returns X",null,null],[14,"tag","","`tag!(&[T]: nom::AsBytes) => &[T] -> IResult<&[T], &[T]>` declares a byte array as a suite to recognize",null,null],[14,"named_args","","Makes a function from a parser combination with arguments.",null,null],[14,"take_until1","","`take_until1!(tag) => &[T] -> IResult<&[T], &[T]>` consumes data until it finds the specified tag",null,null],[14,"is_a_s","","`is_a_s!(&str) => &str -> IResult<&str, &str>` returns the longest list of characters that appear in the provided array",null,null],[14,"terminated","","`terminated!(I -> IResult<I,O>, I -> IResult<I,T>) => I -> IResult<I, O>` terminated(X, closing) returns X",null,null],[14,"fold_many0","","`fold_many0!(I -> IResult<I,O>, R, Fn(R, O) -> R) => I -> IResult<I, R>` Applies the parser 0 or more times and folds the list of return values",null,null],[14,"eof","","`eof!()` returns its input if it is at the end of input data",null,null],[14,"tag_s","","`tag_s!(&str) => &str -> IResult<&str, &str>` declares a string as a suite to recognize",null,null],[14,"separated_pair","","`separated_pair!(I -> IResult<I,O>, I -> IResult<I, T>, I -> IResult<I,P>) => I -> IResult<I, (O,P)>` separated_pair(X,sep,Y) returns (x,y)",null,null],[14,"named_attr","","Makes a function from a parser combination, with attributes",null,null],[14,"peek","","`peek!(I -> IResult<I,O>) => I -> IResult<I, O>` returns a result without consuming the input",null,null],[14,"take_while","","`take_while!(T -> bool) => &[T] -> IResult<&[T], &[T]>` returns the longest list of bytes until the provided function fails.",null,null],[14,"apply_m","","emulate function currying for method calls on structs `apply_m!(self.my_function, arg1, arg2, ...)` becomes `self.my_function(input, arg1, arg2, ...)`",null,null],[14,"is_not","","`is_not!(&[T:AsBytes]) => &[T] -> IResult<&[T], &[T]>` returns the longest list of bytes that do not appear in the provided array",null,null],[14,"take","","`take!(nb) => &[T] -> IResult<&[T], &[T]>` generates a parser consuming the specified number of bytes",null,null],[14,"many_till","","`many_till!(I -> IResult<I,O>, I -> IResult<I,P>) => I -> IResult<I, (Vec<O>, P)>` Applies the first parser until the second applies. Returns a tuple containing the list of results from the first in a Vec and the result of the second.",null,null],[14,"fold_many_m_n","","`fold_many_m_n!(usize, usize, I -> IResult<I,O>, R, Fn(R, O) -> R) => I -> IResult<I, R>` Applies the parser between m and n times (n included) and folds the list of return value",null,null],[14,"consumer_from_parser","","",null,null],[14,"complete","","replaces a `Incomplete` returned by the child parser with an `Error`",null,null],[14,"escaped","","`escaped!(&[T] -> IResult<&[T], &[T]>, T, &[T] -> IResult<&[T], &[T]>) => &[T] -> IResult<&[T], &[T]>` matches a byte string with escaped characters.",null,null],[14,"alt_complete","","Is equivalent to the `alt!` combinator, except that it will not return `Incomplete` when one of the constituting parsers returns `Incomplete`. Instead, it will try the next alternative in the chain.",null,null],[14,"take_str","","`take_str!(nb) => &[T] -> IResult<&[T], &str>` same as take! but returning a &str",null,null],[14,"flat_map","","`flat_map!(R -> IResult<R,S>, S -> IResult<S,T>) => R -> IResult<R, T>`",null,null],[14,"named","","Makes a function from a parser combination",null,null],[14,"switch","","`switch!(I -> IResult<I,P>, P => I -> IResult<I,O> | ... | P => I -> IResult<I,O> ) => I -> IResult<I, O>` choose the next parser depending on the result of the first one, if successful, and returns the result of the second parser",null,null],[14,"one_of","","matches one of the provided characters",null,null],[14,"opt","","`opt!(I -> IResult<I,O>) => I -> IResult<I, Option<O>>` make the underlying parser optional",null,null],[14,"tag_bits","","matches an integer pattern to a bitstream. The number of bits of the input to compare must be specified",null,null],[14,"length_value","","`length_value!(I -> IResult<I, nb>, I -> IResult<I,O>) => I -> IResult<I, Vec<O>>` gets a number from the first parser, takes a subslice of the input of that size, then applies the second parser on that subslice. If the second parser returns `Incomplete`, `length_value` will return an error",null,null],[14,"eat_separator","","helper macros to build a separator parser",null,null],[14,"expr_res","","`expr_res!(Result<E,O>) => I -> IResult<I, O>` evaluate an expression that returns a Result<T,E> and returns a IResult::Done(I,T) if Ok",null,null],[14,"verify","","`verify!(I -> IResult<I,O>, O -> bool) => I -> IResult<I, O>` returns the result of the child parser if it satisfies a verification function",null,null],[14,"recognize","","`recognize!(I -> IResult<I, O> ) => I -> IResult<I, I>` if the child parser was successful, return the consumed input as produced value",null,null],[14,"length_count","","`length_count!(I -> IResult<I, nb>, I -> IResult<I,O>) => I -> IResult<I, Vec<O>>` gets a number from the first parser, then applies the second parser that many times",null,null],[14,"sep","","sep is the parser rewriting macro for whitespace separated formats",null,null],[14,"take_while1","","`take_while1!(T -> bool) => &[T] -> IResult<&[T], &[T]>` returns the longest (non empty) list of bytes until the provided function fails.",null,null],[14,"take_until_either_and_consume","","`take_until_either_and_consume!(tag) => &[T] -> IResult<&[T], &[T]>` consumes data until it finds any of the specified characters, and consume it",null,null],[14,"ws","","`ws!(I -> IResult<I,O>) => I -> IResult<I, O>`",null,null],[14,"apply","","emulate function currying: `apply!(my_function, arg1, arg2, ...)` becomes `my_function(input, arg1, arg2, ...)`",null,null],[14,"take_while1_s","","`take_while1_s!(char -> bool) => &str -> IResult<&str, &str>` returns the longest (non empty) list of characters until the provided function fails.",null,null],[14,"length_data","","`length_data!(I -> IResult<I, nb>) => O`",null,null],[14,"u16","","if the parameter is nom::Endianness::Big, parse a big endian u16 integer, otherwise a little endian u16 integer",null,null],[14,"tag_no_case","","`tag_no_case!(&[T]) => &[T] -> IResult<&[T], &[T]>` declares a case insensitive ascii string as a suite to recognize",null,null],[14,"take_till1","","`take_till1!(T -> bool) => &[T] -> IResult<&[T], &[T]>` returns the longest non empty list of bytes until the provided function succeeds",null,null],[14,"take_until_and_consume","","`take_until_and_consume!(tag) => &[T] -> IResult<&[T], &[T]>` generates a parser consuming bytes until the specified byte sequence is found, and consumes it",null,null],[14,"tag_no_case_s","","`tag_no_case_s!(&str) => &str -> IResult<&str, &str>` declares a case-insensitive string as a suite to recognize",null,null],[14,"error_code","","creates a parse error from a `nom::ErrorKind`",null,null],[14,"opt_res","","`opt_res!(I -> IResult<I,O>) => I -> IResult<I, Result<nom::Err,O>>` make the underlying parser optional",null,null],[14,"pair","","`pair!(I -> IResult<I,O>, I -> IResult<I,P>) => I -> IResult<I, (O,P)>` pair(X,Y), returns (x,y)",null,null],[14,"not","","`not!(I -> IResult<I,O>) => I -> IResult<I, O>` returns a result only if the embedded parser returns Error or Incomplete does not consume the input",null,null],[14,"take_while_s","","`take_while_s!(char -> bool) => &str -> IResult<&str, &str>` returns the longest list of characters until the provided function fails.",null,null],[14,"tuple","","`tuple!(I->IResult<I,A>, I->IResult<I,B>, ... I->IResult<I,X>) => I -> IResult<I, (A, B, ..., X)>` chains parsers and assemble the sub results in a tuple.",null,null],[14,"u32","","if the parameter is nom::Endianness::Big, parse a big endian u32 integer, otherwise a little endian u32 integer",null,null],[14,"many_m_n","","`many_m_n!(usize, usize, I -> IResult<I,O>) => I -> IResult<I, Vec<O>>` Applies the parser between m and n times (n included) and returns the list of results in a Vec",null,null],[14,"many0","","`many0!(I -> IResult<I,O>) => I -> IResult<I, Vec<O>>` Applies the parser 0 or more times and returns the list of results in a Vec",null,null],[14,"length_bytes","","`length_bytes!(&[T] -> IResult<&[T], nb>) => &[T] -> IResult<&[T], &[T]>` Gets a number from the first parser, then extracts that many bytes from the remaining stream",null,null],[14,"value","","`value!(T, R -> IResult<R, S> ) => R -> IResult<R, T>`",null,null],[14,"u64","","if the parameter is nom::Endianness::Big, parse a big endian u64 integer, otherwise a little endian u64 integer",null,null],[14,"take_till1_s","","`take_till1_s!(char -> bool) => &str -> IResult<&str, &str>` returns the longest non empty list of characters until the provided function succeeds",null,null],[14,"fix_error","","translate parser result from IResult<I,O,u32> to IResult<I,O,E> with a custom type",null,null],[14,"return_error","","Prevents backtracking if the child parser fails",null,null],[14,"cond_with_error","","`cond_with_error!(bool, I -> IResult<I,O>) => I -> IResult<I, Option<O>>` Conditional combinator",null,null],[14,"separated_list","","`separated_list!(I -> IResult<I,T>, I -> IResult<I,O>) => I -> IResult<I, Vec<O>>` separated_list(sep, X) returns Vec will return Incomplete if there may be more elements",null,null],[14,"add_return_error","","Add an error if the child parser fails",null,null],[14,"cond","","`cond!(bool, I -> IResult<I,O>) => I -> IResult<I, Option<O>>` Conditional combinator",null,null],[14,"map_res","","`map_res!(I -> IResult<I,O>, O -> Result<P>) => I -> IResult<I, P>` maps a function returning a Result on the output of a parser",null,null],[14,"call","","Used to wrap common expressions and function as macros",null,null],[14,"compiler_error","","",null,null],[14,"permutation","","`permutation!(I -> IResult<I,A>, I -> IResult<I,B>, ... I -> IResult<I,X> ) => I -> IResult<I, (A,B,...X)>` applies its sub parsers in a sequence, but independent from their order this parser will only succeed if all of its sub parsers succeed",null,null],[14,"map","","`map!(I -> IResult<I,O>, O -> P) => I -> IResult<I, P>` maps a function on the result of a parser",null,null],[14,"separated_nonempty_list","","`separated_nonempty_list!(I -> IResult<I,T>, I -> IResult<I,O>) => I -> IResult<I, Vec<O>>` separated_nonempty_list(sep, X) returns Vec will return Incomplete if there may be more elements",null,null],[14,"separated_nonempty_list_complete","","`separated_nonempty_list_complete!(I -> IResult<I,T>, I -> IResult<I,O>) => I -> IResult<I, Vec<O>>` This is equivalent to the `separated_nonempty_list!` combinator, except that it will return `Error` when either the separator or element subparser returns `Incomplete`.",null,null],[14,"take_until_s","","`take_until_s!(&str) => &str -> IResult<&str, &str>` generates a parser consuming all chars until the specified string is found and leaves it in the remaining input",null,null],[14,"parse_to","","`parse_to!(O) => I -> IResult<I, O>` uses the `parse` method from `std::str::FromStr` to convert the current input to the specified type",null,null],[14,"many1","","`many1!(I -> IResult<I,O>) => I -> IResult<I, Vec<O>>` Applies the parser 1 or more times and returns the list of results in a Vec",null,null],[14,"method","","Makes a method from a parser combination",null,null],[14,"count","","`count!(I -> IResult<I,O>, nb) => I -> IResult<I, Vec<O>>` Applies the child parser a specified number of times",null,null],[14,"expr_opt","","`expr_opt!(Option<O>) => I -> IResult<I, O>` evaluate an expression that returns a Option and returns a IResult::Done(I,T) if Some",null,null],[14,"separated_list_complete","","`separated_list_complete!(I -> IResult<I,T>, I -> IResult<I,O>) => I -> IResult<I, Vec<O>>` This is equivalent to the `separated_list!` combinator, except that it will return `Error` when either the separator or element subparser returns `Incomplete`.",null,null],[14,"none_of","","matches anything but the provided characters",null,null],[14,"do_parse","","`do_parse!(I->IResult<I,A> >> I->IResult<I,B> >> ... I->IResult<I,X> , ( O ) ) => I -> IResult<I, O>` do_parse applies sub parsers in a sequence. it can store intermediary results and make them available for later parsers",null,null],[14,"call_m","","Used to called methods then move self back into self",null,null],[14,"take_till","","`take_till!(T -> bool) => &[T] -> IResult<&[T], &[T]>` returns the longest list of bytes until the provided function succeeds",null,null],[14,"take_until_either","","`take_until_either!(tag) => &[T] -> IResult<&[T], &[T]>`",null,null],[14,"take_till_s","","`take_till_s!(char -> bool) => &str -> IResult<&str, &str>` returns the longest list of characters until the provided function succeeds",null,null],[14,"fold_many1","","`fold_many1!(I -> IResult<I,O>, R, Fn(R, O) -> R) => I -> IResult<I, R>` Applies the parser 1 or more times and folds the list of return values",null,null],[14,"is_a","","`is_a!(&[T]) => &[T] -> IResult<&[T], &[T]>` returns the longest list of bytes that appear in the provided array",null,null],[14,"take_until_and_consume1","","`take_until_and_consume1!(tag) => &[T] -> IResult<&[T], &[T]>` generates a parser consuming bytes (at least 1) until the specified byte sequence is found, and consumes it",null,null],[14,"take_until","","`take_until!(tag) => &[T] -> IResult<&[T], &[T]>` consumes data until it finds the specified tag",null,null],[14,"error_node_position","","creates a parse error from a `nom::ErrorKind`, the position in the input and the next error in the parsing tree. if \"verbose-errors\" is not activated, it default to only the error code",null,null],[14,"map_opt","","`map_opt!(I -> IResult<I,O>, O -> Option<P>) => I -> IResult<I, P>` maps a function returning an Option on the output of a parser",null,null],[14,"delimited","","`delimited!(I -> IResult<I,T>, I -> IResult<I,O>, I -> IResult<I,U>) => I -> IResult<I, O>` delimited(opening, X, closing) returns X",null,null],[14,"i16","","if the parameter is nom::Endianness::Big, parse a big endian i16 integer, otherwise a little endian i16 integer",null,null],[14,"take_s","","`take_s!(nb) => &str -> IResult<&str, &str>` generates a parser consuming the specified number of characters",null,null],[14,"count_fixed","","`count_fixed!(O, I -> IResult<I,O>, nb) => I -> IResult<I, [O; nb]>` Applies the child parser a fixed number of times and returns a fixed size array The type must be specified and it must be `Copy`",null,null],[14,"error_node","","creates a parse error from a `nom::ErrorKind` and the next error in the parsing tree. if \"verbose-errors\" is not activated, it default to only the error code",null,null],[14,"take_until_and_consume_s","","`take_until_and_consume_s!(&str) => &str -> IResult<&str, &str>` generates a parser consuming all chars until the specified string is found and consumes it",null,null],[14,"i64","","if the parameter is nom::Endianness::Big, parse a big endian i64 integer, otherwise a little endian i64 integer",null,null],[14,"escaped_transform","","`escaped_transform!(&[T] -> IResult<&[T], &[T]>, T, &[T] -> IResult<&[T], &[T]>) => &[T] -> IResult<&[T], Vec<T>>` matches a byte string with escaped characters.",null,null],[14,"char","","matches one character: `char!(char) => &[u8] -> IResult<&[u8], char>",null,null],[14,"dbg","","Prints a message if the parser fails",null,null],[14,"take_bits","","`take_bits!(type, nb) => ( (&[T], usize), U, usize) -> IResult<(&[T], usize), U>` generates a parser consuming the specified number of bits.",null,null],[14,"wrap_sep","","",null,null],[14,"error_position","","creates a parse error from a `nom::ErrorKind` and the position in the input if \"verbose-errors\" is not activated, it default to only the error code",null,null],[14,"try_parse","","A bit like `std::try!`, this macro will return the remaining input and parsed value if the child parser returned `Done`, and will do an early return for `Error` and `Incomplete` this can provide more flexibility than `do_parse!` if needed",null,null],[14,"bytes","","Counterpart to bits, `bytes!( parser ) => ( (&[u8], usize), &[u8] -> IResult<&[u8], T> ) -> IResult<(&[u8], usize), T>`, transforms its bits stream input into a byte slice for the underlying parsers. If we start in the middle of a byte throws away the bits until the end of the byte.",null,null],[14,"closure","","Wraps a parser in a closure",null,null],[14,"alt","","Try a list of parsers and return the result of the first successful one",null,null],[14,"bits","","`bits!( parser ) => ( &[u8], (&[u8], usize) -> IResult<(&[u8], usize), T> ) -> IResult<&[u8], T>` transforms its byte slice input into a bit stream for the underlying parsers",null,null],[14,"cond_reduce","","`cond_reduce!(bool, I -> IResult<I,O>) => I -> IResult<I, O>` Conditional combinator with error",null,null],[14,"tap","","`tap!(name: I -> IResult<I,O> => { block }) => I -> IResult<I, O>` allows access to the parser's result without affecting it",null,null],[14,"is_not_s","","`is_not_s!(&str) => &str -> IResult<&str, &str>` returns the longest list of characters that do not appear in the provided array",null,null]],"paths":[]};
searchIndex["mccs_db"] = {"doc":"Monitor Command Control Set VCP feature code meanings and data interpretation.","items":[[3,"Descriptor","mccs_db","Describes a VCP feature code's functionality and value format.",null,null],[12,"name","","The name of the feature.",0,null],[12,"description","","A detailed description of the feature.",0,null],[12,"group","","The MCCS grouping this feature belongs to.",0,null],[12,"code","","The VCP code of the feature.",0,null],[12,"ty","","The data type of the feature.",0,null],[12,"access","","Whether the feature can be set, read, or both.",0,null],[12,"mandatory","","Whether the feature is required to be supported by the display for MCCS specification compliance.",0,null],[12,"interacts_with","","Any other feature codes that this \"interacts\" with.",0,null],[3,"Database","","Describes all the VCP feature codes supported by an MCCS specification or display.",null,null],[4,"TableInterpretation","","Describes how to interpret a table's raw value.",null,null],[13,"Generic","","Generic unparsed data.",1,null],[13,"CodePage","","First byte is the code page where `0x00` is the default.",1,null],[4,"ValueInterpretation","","Describes how to interpret a value's raw value.",null,null],[13,"Continuous","","Generic unparsed data.",2,null],[13,"NonContinuous","","Generic unparsed data.",2,null],[13,"NonZeroWrite","","Must be set to a non-zero value in order to run the operation.",2,null],[13,"VcpVersion","","MCCS version is returned in `mh` (major version) and `ml` (minor/revision).",2,null],[4,"ValueType","","Describes the type of a VCP value and how to interpret it.",null,null],[13,"Unknown","","The type of the data is not known",3,null],[13,"Continuous","","The data is a continuous value.",3,null],[12,"interpretation","mccs_db::ValueType","Describes how to interpret the continuous value.",3,null],[13,"NonContinuous","mccs_db","The data is a non-continuous value.",3,null],[12,"values","mccs_db::ValueType","The values allowed or supported to be set, as well as their user-facing names.",3,null],[12,"interpretation","","Describes how to interpret the non-continuous value.",3,null],[13,"Table","mccs_db","The data is a table (byte array)",3,null],[12,"interpretation","mccs_db::ValueType","Describes how to interpret the table.",3,null],[4,"Access","mccs_db","The operations allowed on a given VCP feature code.",null,null],[13,"ReadOnly","","The value can only be read from.",4,null],[13,"WriteOnly","","The value can only be written to.",4,null],[13,"ReadWrite","","The value is both readwritable.",4,null],[11,"fmt","","",1,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",1,{"inputs":[{"name":"self"}],"output":{"name":"tableinterpretation"}}],[11,"eq","","",1,{"inputs":[{"name":"self"},{"name":"tableinterpretation"}],"output":{"name":"bool"}}],[11,"partial_cmp","","",1,{"inputs":[{"name":"self"},{"name":"tableinterpretation"}],"output":{"generics":["ordering"],"name":"option"}}],[11,"cmp","","",1,{"inputs":[{"name":"self"},{"name":"tableinterpretation"}],"output":{"name":"ordering"}}],[11,"hash","","",1,null],[11,"default","","",1,{"inputs":[],"output":{"name":"self"}}],[11,"format","","Formats a table for user display.",1,null],[11,"fmt","","",2,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",2,{"inputs":[{"name":"self"}],"output":{"name":"valueinterpretation"}}],[11,"eq","","",2,{"inputs":[{"name":"self"},{"name":"valueinterpretation"}],"output":{"name":"bool"}}],[11,"partial_cmp","","",2,{"inputs":[{"name":"self"},{"name":"valueinterpretation"}],"output":{"generics":["ordering"],"name":"option"}}],[11,"cmp","","",2,{"inputs":[{"name":"self"},{"name":"valueinterpretation"}],"output":{"name":"ordering"}}],[11,"hash","","",2,null],[11,"format","","Formats a value for user display.",2,{"inputs":[{"name":"self"},{"name":"value"}],"output":{"name":"string"}}],[11,"fmt","","",3,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",3,{"inputs":[{"name":"self"}],"output":{"name":"valuetype"}}],[11,"eq","","",3,{"inputs":[{"name":"self"},{"name":"valuetype"}],"output":{"name":"bool"}}],[11,"ne","","",3,{"inputs":[{"name":"self"},{"name":"valuetype"}],"output":{"name":"bool"}}],[11,"partial_cmp","","",3,{"inputs":[{"name":"self"},{"name":"valuetype"}],"output":{"generics":["ordering"],"name":"option"}}],[11,"lt","","",3,{"inputs":[{"name":"self"},{"name":"valuetype"}],"output":{"name":"bool"}}],[11,"le","","",3,{"inputs":[{"name":"self"},{"name":"valuetype"}],"output":{"name":"bool"}}],[11,"gt","","",3,{"inputs":[{"name":"self"},{"name":"valuetype"}],"output":{"name":"bool"}}],[11,"ge","","",3,{"inputs":[{"name":"self"},{"name":"valuetype"}],"output":{"name":"bool"}}],[11,"cmp","","",3,{"inputs":[{"name":"self"},{"name":"valuetype"}],"output":{"name":"ordering"}}],[11,"hash","","",3,null],[11,"default","","",3,{"inputs":[],"output":{"name":"self"}}],[11,"fmt","","",4,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",4,{"inputs":[{"name":"self"}],"output":{"name":"access"}}],[11,"eq","","",4,{"inputs":[{"name":"self"},{"name":"access"}],"output":{"name":"bool"}}],[11,"partial_cmp","","",4,{"inputs":[{"name":"self"},{"name":"access"}],"output":{"generics":["ordering"],"name":"option"}}],[11,"cmp","","",4,{"inputs":[{"name":"self"},{"name":"access"}],"output":{"name":"ordering"}}],[11,"hash","","",4,null],[11,"default","","",4,{"inputs":[],"output":{"name":"self"}}],[11,"fmt","","",0,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"default","","",0,{"inputs":[],"output":{"name":"descriptor"}}],[11,"clone","","",0,{"inputs":[{"name":"self"}],"output":{"name":"descriptor"}}],[11,"fmt","","",5,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",5,{"inputs":[{"name":"self"}],"output":{"name":"database"}}],[11,"default","","",5,{"inputs":[],"output":{"name":"database"}}],[11,"from_version","","Create a new database from a specified MCCS specification version.",5,{"inputs":[{"name":"version"}],"output":{"name":"self"}}],[11,"from_database","","Create a new database from a specified database description YAML file.",5,{"inputs":[{"name":"r"},{"name":"version"}],"output":{"name":"result"}}],[11,"apply_capabilities","","Filter out any feature codes or values that are not supported by the specified display.",5,{"inputs":[{"name":"self"},{"name":"capabilities"}],"output":null}],[11,"get","","Get the description of a given VCP feature code.",5,{"inputs":[{"name":"self"},{"name":"featurecode"}],"output":{"generics":["descriptor"],"name":"option"}}]],"paths":[[3,"Descriptor"],[4,"TableInterpretation"],[4,"ValueInterpretation"],[4,"ValueType"],[4,"Access"],[3,"Database"]]};
initSearch(searchIndex);
