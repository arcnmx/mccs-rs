var searchIndex = {};
searchIndex["mccs"] = {"doc":"VESA Monitor Command Control Set standardizes the meaning of DDC/CI VCP feature codes, and allows a display to broadcast its capabilities to the host.","items":[[3,"Value","mccs","VCP Value",null,null],[12,"ty","","Specifies the type of the value, continuous or non-continuous.",0,null],[12,"mh","","The high byte of the maximum allowed value.",0,null],[12,"ml","","The low byte of the maximum allowed value.",0,null],[12,"sh","","The high byte of the value.",0,null],[12,"sl","","The low byte of the value.",0,null],[3,"Capabilities","","Parsed display capabilities string.",null,null],[12,"protocol","","The protocol class.",1,null],[12,"ty","","The type of display.",1,null],[12,"model","","The model name/number of the display.",1,null],[12,"commands","","A list of the supported VCP commands.",1,null],[12,"ms_whql","","A value of `1` seems to indicate that the monitor has passed Microsoft's Windows Hardware Quality Labs testing.",1,null],[12,"mccs_version","","Monitor Command Control Set version code.",1,null],[12,"vcp_features","","Virtual Control Panel feature code descriptors.",1,null],[12,"edid","","Extended Display Identification Data",1,null],[12,"vdif","","Video Display Information Format are optional extension blocks for the EDID. Like the EDID field, this is probably not in use.",1,null],[12,"unknown_tags","","Additional unrecognized data from the capability string.",1,null],[3,"Version","","Monitor Command Control Set specification version code",null,null],[12,"major","","Major version number",2,null],[12,"minor","","Minor revision version",2,null],[3,"VcpDescriptor","","Descriptive information about a supported VCP feature code.",null,null],[12,"name","","The name of the feature code, if different from the standard MCCS spec.",3,null],[12,"values","","Allowed values for this feature, and optionally their names.",3,null],[3,"UnknownTag","","An unrecognized entry in the capability string",null,null],[12,"name","","The name of the entry",4,null],[12,"data","","The data contained in the entry, usually an unparsed string.",4,null],[4,"ValueType","","VCP feature type.",null,null],[13,"SetParameter","","Sending a command of this type changes some aspect of the monitor's operation.",5,null],[13,"Momentary","","Sending a command of this type causes the monitor to initiate a self-timed operation and then revert to its original state.",5,null],[4,"Protocol","","Display protocol class",null,null],[13,"Monitor","","Standard monitor",6,null],[13,"Display","","I have never seen this outside of an MCCS spec example, it may be a typo.",6,null],[13,"Unknown","","Unrecognized protocol class",6,null],[4,"Type","","Display type",null,null],[13,"Crt","","Cathode Ray Tube display",7,null],[13,"Lcd","","Liquid Crystal Display",7,null],[13,"Led","","Also an LCD, I'm not sure this should exist.",7,null],[13,"Unknown","","Unrecognized display type",7,null],[4,"UnknownData","","Data that can be contained in a capability entry.",null,null],[13,"String","","UTF-8/ASCII data",8,null],[13,"StringBytes","","Data that is not valid UTF-8",8,null],[13,"Binary","","Length-prefixed binary data",8,null],[6,"FeatureCode","","VCP feature code",null,null],[6,"EdidData","","Extended Display Identification Data",null,null],[6,"VdifData","","Video Display Information Format",null,null],[6,"ValueNames","","VCP feature value names",null,null],[11,"clone","","",0,{"inputs":[{"name":"self"}],"output":{"name":"value"}}],[11,"default","","",0,{"inputs":[],"output":{"name":"value"}}],[11,"eq","","",0,{"inputs":[{"name":"self"},{"name":"value"}],"output":{"name":"bool"}}],[11,"ne","","",0,{"inputs":[{"name":"self"},{"name":"value"}],"output":{"name":"bool"}}],[11,"partial_cmp","","",0,{"inputs":[{"name":"self"},{"name":"value"}],"output":{"generics":["ordering"],"name":"option"}}],[11,"lt","","",0,{"inputs":[{"name":"self"},{"name":"value"}],"output":{"name":"bool"}}],[11,"le","","",0,{"inputs":[{"name":"self"},{"name":"value"}],"output":{"name":"bool"}}],[11,"gt","","",0,{"inputs":[{"name":"self"},{"name":"value"}],"output":{"name":"bool"}}],[11,"ge","","",0,{"inputs":[{"name":"self"},{"name":"value"}],"output":{"name":"bool"}}],[11,"cmp","","",0,{"inputs":[{"name":"self"},{"name":"value"}],"output":{"name":"ordering"}}],[11,"hash","","",0,null],[11,"from_value","","Create a new `Value` from a scalar value.",0,{"inputs":[{"name":"u16"}],"output":{"name":"self"}}],[11,"value","","Combines the value bytes into a single value.",0,{"inputs":[{"name":"self"}],"output":{"name":"u16"}}],[11,"maximum","","Combines the maximum value bytes into a single value.",0,{"inputs":[{"name":"self"}],"output":{"name":"u16"}}],[11,"ty","","VCP feature type, if recognized.",0,{"inputs":[{"name":"self"}],"output":{"generics":["valuetype","u8"],"name":"result"}}],[11,"fmt","","",0,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"fmt","","",1,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"default","","",1,{"inputs":[],"output":{"name":"capabilities"}}],[11,"clone","","",1,{"inputs":[{"name":"self"}],"output":{"name":"capabilities"}}],[11,"eq","","",1,{"inputs":[{"name":"self"},{"name":"capabilities"}],"output":{"name":"bool"}}],[11,"ne","","",1,{"inputs":[{"name":"self"},{"name":"capabilities"}],"output":{"name":"bool"}}],[11,"partial_cmp","","",1,{"inputs":[{"name":"self"},{"name":"capabilities"}],"output":{"generics":["ordering"],"name":"option"}}],[11,"lt","","",1,{"inputs":[{"name":"self"},{"name":"capabilities"}],"output":{"name":"bool"}}],[11,"le","","",1,{"inputs":[{"name":"self"},{"name":"capabilities"}],"output":{"name":"bool"}}],[11,"gt","","",1,{"inputs":[{"name":"self"},{"name":"capabilities"}],"output":{"name":"bool"}}],[11,"ge","","",1,{"inputs":[{"name":"self"},{"name":"capabilities"}],"output":{"name":"bool"}}],[11,"cmp","","",1,{"inputs":[{"name":"self"},{"name":"capabilities"}],"output":{"name":"ordering"}}],[11,"hash","","",1,null],[11,"fmt","","",6,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",6,{"inputs":[{"name":"self"}],"output":{"name":"protocol"}}],[11,"eq","","",6,{"inputs":[{"name":"self"},{"name":"protocol"}],"output":{"name":"bool"}}],[11,"ne","","",6,{"inputs":[{"name":"self"},{"name":"protocol"}],"output":{"name":"bool"}}],[11,"partial_cmp","","",6,{"inputs":[{"name":"self"},{"name":"protocol"}],"output":{"generics":["ordering"],"name":"option"}}],[11,"lt","","",6,{"inputs":[{"name":"self"},{"name":"protocol"}],"output":{"name":"bool"}}],[11,"le","","",6,{"inputs":[{"name":"self"},{"name":"protocol"}],"output":{"name":"bool"}}],[11,"gt","","",6,{"inputs":[{"name":"self"},{"name":"protocol"}],"output":{"name":"bool"}}],[11,"ge","","",6,{"inputs":[{"name":"self"},{"name":"protocol"}],"output":{"name":"bool"}}],[11,"cmp","","",6,{"inputs":[{"name":"self"},{"name":"protocol"}],"output":{"name":"ordering"}}],[11,"hash","","",6,null],[11,"from","","",6,{"inputs":[{"name":"str"}],"output":{"name":"self"}}],[11,"fmt","","",6,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"from_str","","",6,{"inputs":[{"name":"str"}],"output":{"name":"result"}}],[11,"fmt","","",7,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",7,{"inputs":[{"name":"self"}],"output":{"name":"type"}}],[11,"eq","","",7,{"inputs":[{"name":"self"},{"name":"type"}],"output":{"name":"bool"}}],[11,"ne","","",7,{"inputs":[{"name":"self"},{"name":"type"}],"output":{"name":"bool"}}],[11,"partial_cmp","","",7,{"inputs":[{"name":"self"},{"name":"type"}],"output":{"generics":["ordering"],"name":"option"}}],[11,"lt","","",7,{"inputs":[{"name":"self"},{"name":"type"}],"output":{"name":"bool"}}],[11,"le","","",7,{"inputs":[{"name":"self"},{"name":"type"}],"output":{"name":"bool"}}],[11,"gt","","",7,{"inputs":[{"name":"self"},{"name":"type"}],"output":{"name":"bool"}}],[11,"ge","","",7,{"inputs":[{"name":"self"},{"name":"type"}],"output":{"name":"bool"}}],[11,"cmp","","",7,{"inputs":[{"name":"self"},{"name":"type"}],"output":{"name":"ordering"}}],[11,"hash","","",7,null],[11,"from","","",7,{"inputs":[{"name":"str"}],"output":{"name":"self"}}],[11,"fmt","","",7,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"from_str","","",7,{"inputs":[{"name":"str"}],"output":{"name":"result"}}],[11,"fmt","","",2,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"default","","",2,{"inputs":[],"output":{"name":"version"}}],[11,"clone","","",2,{"inputs":[{"name":"self"}],"output":{"name":"version"}}],[11,"eq","","",2,{"inputs":[{"name":"self"},{"name":"version"}],"output":{"name":"bool"}}],[11,"ne","","",2,{"inputs":[{"name":"self"},{"name":"version"}],"output":{"name":"bool"}}],[11,"partial_cmp","","",2,{"inputs":[{"name":"self"},{"name":"version"}],"output":{"generics":["ordering"],"name":"option"}}],[11,"lt","","",2,{"inputs":[{"name":"self"},{"name":"version"}],"output":{"name":"bool"}}],[11,"le","","",2,{"inputs":[{"name":"self"},{"name":"version"}],"output":{"name":"bool"}}],[11,"gt","","",2,{"inputs":[{"name":"self"},{"name":"version"}],"output":{"name":"bool"}}],[11,"ge","","",2,{"inputs":[{"name":"self"},{"name":"version"}],"output":{"name":"bool"}}],[11,"cmp","","",2,{"inputs":[{"name":"self"},{"name":"version"}],"output":{"name":"ordering"}}],[11,"hash","","",2,null],[11,"new","","Create a new MCCS version from the specified version and revision.",2,{"inputs":[{"name":"u8"},{"name":"u8"}],"output":{"name":"self"}}],[11,"fmt","","",3,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"default","","",3,{"inputs":[],"output":{"name":"vcpdescriptor"}}],[11,"clone","","",3,{"inputs":[{"name":"self"}],"output":{"name":"vcpdescriptor"}}],[11,"eq","","",3,{"inputs":[{"name":"self"},{"name":"vcpdescriptor"}],"output":{"name":"bool"}}],[11,"ne","","",3,{"inputs":[{"name":"self"},{"name":"vcpdescriptor"}],"output":{"name":"bool"}}],[11,"partial_cmp","","",3,{"inputs":[{"name":"self"},{"name":"vcpdescriptor"}],"output":{"generics":["ordering"],"name":"option"}}],[11,"lt","","",3,{"inputs":[{"name":"self"},{"name":"vcpdescriptor"}],"output":{"name":"bool"}}],[11,"le","","",3,{"inputs":[{"name":"self"},{"name":"vcpdescriptor"}],"output":{"name":"bool"}}],[11,"gt","","",3,{"inputs":[{"name":"self"},{"name":"vcpdescriptor"}],"output":{"name":"bool"}}],[11,"ge","","",3,{"inputs":[{"name":"self"},{"name":"vcpdescriptor"}],"output":{"name":"bool"}}],[11,"cmp","","",3,{"inputs":[{"name":"self"},{"name":"vcpdescriptor"}],"output":{"name":"ordering"}}],[11,"hash","","",3,null],[11,"values","","The allowed values for this feature code.",3,{"inputs":[{"name":"self"}],"output":{"generics":["u8","option"],"name":"keys"}}],[11,"fmt","","",4,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",4,{"inputs":[{"name":"self"}],"output":{"name":"unknowntag"}}],[11,"eq","","",4,{"inputs":[{"name":"self"},{"name":"unknowntag"}],"output":{"name":"bool"}}],[11,"ne","","",4,{"inputs":[{"name":"self"},{"name":"unknowntag"}],"output":{"name":"bool"}}],[11,"partial_cmp","","",4,{"inputs":[{"name":"self"},{"name":"unknowntag"}],"output":{"generics":["ordering"],"name":"option"}}],[11,"lt","","",4,{"inputs":[{"name":"self"},{"name":"unknowntag"}],"output":{"name":"bool"}}],[11,"le","","",4,{"inputs":[{"name":"self"},{"name":"unknowntag"}],"output":{"name":"bool"}}],[11,"gt","","",4,{"inputs":[{"name":"self"},{"name":"unknowntag"}],"output":{"name":"bool"}}],[11,"ge","","",4,{"inputs":[{"name":"self"},{"name":"unknowntag"}],"output":{"name":"bool"}}],[11,"cmp","","",4,{"inputs":[{"name":"self"},{"name":"unknowntag"}],"output":{"name":"ordering"}}],[11,"hash","","",4,null],[11,"fmt","","",8,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",8,{"inputs":[{"name":"self"}],"output":{"name":"unknowndata"}}],[11,"eq","","",8,{"inputs":[{"name":"self"},{"name":"unknowndata"}],"output":{"name":"bool"}}],[11,"ne","","",8,{"inputs":[{"name":"self"},{"name":"unknowndata"}],"output":{"name":"bool"}}],[11,"partial_cmp","","",8,{"inputs":[{"name":"self"},{"name":"unknowndata"}],"output":{"generics":["ordering"],"name":"option"}}],[11,"lt","","",8,{"inputs":[{"name":"self"},{"name":"unknowndata"}],"output":{"name":"bool"}}],[11,"le","","",8,{"inputs":[{"name":"self"},{"name":"unknowndata"}],"output":{"name":"bool"}}],[11,"gt","","",8,{"inputs":[{"name":"self"},{"name":"unknowndata"}],"output":{"name":"bool"}}],[11,"ge","","",8,{"inputs":[{"name":"self"},{"name":"unknowndata"}],"output":{"name":"bool"}}],[11,"cmp","","",8,{"inputs":[{"name":"self"},{"name":"unknowndata"}],"output":{"name":"ordering"}}],[11,"hash","","",8,null]],"paths":[[3,"Value"],[3,"Capabilities"],[3,"Version"],[3,"VcpDescriptor"],[3,"UnknownTag"],[4,"ValueType"],[4,"Protocol"],[4,"Type"],[4,"UnknownData"]]};
searchIndex["mccs_caps"] = {"doc":"MCCS compliant displays will report their supported capabilities in a string retrieved over DDC/CI. The format of this string is specified in the DDC specification, MCCS, and ACCESS.bus section 7. This crate parses the capability string into structured data.","items":[[5,"parse_capabilities","mccs_caps","Parses a MCCS capability string.",null,{"inputs":[{"name":"c"}],"output":{"generics":["capabilities"],"name":"result"}}]],"paths":[]};
searchIndex["mccs_db"] = {"doc":"Monitor Command Control Set VCP feature code meanings and data interpretation.","items":[[3,"Descriptor","mccs_db","Describes a VCP feature code's functionality and value format.",null,null],[12,"name","","The name of the feature.",0,null],[12,"description","","A detailed description of the feature.",0,null],[12,"group","","The MCCS grouping this feature belongs to.",0,null],[12,"code","","The VCP code of the feature.",0,null],[12,"ty","","The data type of the feature.",0,null],[12,"access","","Whether the feature can be set, read, or both.",0,null],[12,"mandatory","","Whether the feature is required to be supported by the display for MCCS specification compliance.",0,null],[12,"interacts_with","","Any other feature codes that this \"interacts\" with.",0,null],[3,"Database","","Describes all the VCP feature codes supported by an MCCS specification or display.",null,null],[4,"TableInterpretation","","Describes how to interpret a table's raw value.",null,null],[13,"Generic","","Generic unparsed data.",1,null],[13,"CodePage","","First byte is the code page where `0x00` is the default.",1,null],[4,"ValueInterpretation","","Describes how to interpret a value's raw value.",null,null],[13,"Continuous","","Generic unparsed data.",2,null],[13,"NonContinuous","","Generic unparsed data.",2,null],[13,"NonZeroWrite","","Must be set to a non-zero value in order to run the operation.",2,null],[13,"VcpVersion","","MCCS version is returned in `mh` (major version) and `ml` (minor/revision).",2,null],[4,"ValueType","","Describes the type of a VCP value and how to interpret it.",null,null],[13,"Unknown","","The type of the data is not known",3,null],[13,"Continuous","","The data is a continuous value.",3,null],[12,"interpretation","mccs_db::ValueType","Describes how to interpret the continuous value.",3,null],[13,"NonContinuous","mccs_db","The data is a non-continuous value.",3,null],[12,"values","mccs_db::ValueType","The values allowed or supported to be set, as well as their user-facing names.",3,null],[12,"interpretation","","Describes how to interpret the non-continuous value.",3,null],[13,"Table","mccs_db","The data is a table (byte array)",3,null],[12,"interpretation","mccs_db::ValueType","Describes how to interpret the table.",3,null],[4,"Access","mccs_db","The operations allowed on a given VCP feature code.",null,null],[13,"ReadOnly","","The value can only be read from.",4,null],[13,"WriteOnly","","The value can only be written to.",4,null],[13,"ReadWrite","","The value is both readwritable.",4,null],[11,"fmt","","",1,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",1,{"inputs":[{"name":"self"}],"output":{"name":"tableinterpretation"}}],[11,"eq","","",1,{"inputs":[{"name":"self"},{"name":"tableinterpretation"}],"output":{"name":"bool"}}],[11,"partial_cmp","","",1,{"inputs":[{"name":"self"},{"name":"tableinterpretation"}],"output":{"generics":["ordering"],"name":"option"}}],[11,"cmp","","",1,{"inputs":[{"name":"self"},{"name":"tableinterpretation"}],"output":{"name":"ordering"}}],[11,"hash","","",1,null],[11,"default","","",1,{"inputs":[],"output":{"name":"self"}}],[11,"format","","Formats a table for user display.",1,null],[11,"fmt","","",2,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",2,{"inputs":[{"name":"self"}],"output":{"name":"valueinterpretation"}}],[11,"eq","","",2,{"inputs":[{"name":"self"},{"name":"valueinterpretation"}],"output":{"name":"bool"}}],[11,"partial_cmp","","",2,{"inputs":[{"name":"self"},{"name":"valueinterpretation"}],"output":{"generics":["ordering"],"name":"option"}}],[11,"cmp","","",2,{"inputs":[{"name":"self"},{"name":"valueinterpretation"}],"output":{"name":"ordering"}}],[11,"hash","","",2,null],[11,"format","","Formats a value for user display.",2,{"inputs":[{"name":"self"},{"name":"value"}],"output":{"name":"string"}}],[11,"fmt","","",3,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",3,{"inputs":[{"name":"self"}],"output":{"name":"valuetype"}}],[11,"eq","","",3,{"inputs":[{"name":"self"},{"name":"valuetype"}],"output":{"name":"bool"}}],[11,"ne","","",3,{"inputs":[{"name":"self"},{"name":"valuetype"}],"output":{"name":"bool"}}],[11,"partial_cmp","","",3,{"inputs":[{"name":"self"},{"name":"valuetype"}],"output":{"generics":["ordering"],"name":"option"}}],[11,"lt","","",3,{"inputs":[{"name":"self"},{"name":"valuetype"}],"output":{"name":"bool"}}],[11,"le","","",3,{"inputs":[{"name":"self"},{"name":"valuetype"}],"output":{"name":"bool"}}],[11,"gt","","",3,{"inputs":[{"name":"self"},{"name":"valuetype"}],"output":{"name":"bool"}}],[11,"ge","","",3,{"inputs":[{"name":"self"},{"name":"valuetype"}],"output":{"name":"bool"}}],[11,"cmp","","",3,{"inputs":[{"name":"self"},{"name":"valuetype"}],"output":{"name":"ordering"}}],[11,"hash","","",3,null],[11,"default","","",3,{"inputs":[],"output":{"name":"self"}}],[11,"fmt","","",4,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",4,{"inputs":[{"name":"self"}],"output":{"name":"access"}}],[11,"eq","","",4,{"inputs":[{"name":"self"},{"name":"access"}],"output":{"name":"bool"}}],[11,"partial_cmp","","",4,{"inputs":[{"name":"self"},{"name":"access"}],"output":{"generics":["ordering"],"name":"option"}}],[11,"cmp","","",4,{"inputs":[{"name":"self"},{"name":"access"}],"output":{"name":"ordering"}}],[11,"hash","","",4,null],[11,"default","","",4,{"inputs":[],"output":{"name":"self"}}],[11,"fmt","","",0,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"default","","",0,{"inputs":[],"output":{"name":"descriptor"}}],[11,"clone","","",0,{"inputs":[{"name":"self"}],"output":{"name":"descriptor"}}],[11,"fmt","","",5,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",5,{"inputs":[{"name":"self"}],"output":{"name":"database"}}],[11,"default","","",5,{"inputs":[],"output":{"name":"database"}}],[11,"from_version","","Create a new database from a specified MCCS specification version.",5,{"inputs":[{"name":"version"}],"output":{"name":"self"}}],[11,"from_database","","Create a new database from a specified database description YAML file.",5,{"inputs":[{"name":"r"},{"name":"version"}],"output":{"name":"result"}}],[11,"apply_capabilities","","Filter out any feature codes or values that are not supported by the specified display.",5,{"inputs":[{"name":"self"},{"name":"capabilities"}],"output":null}],[11,"get","","Get the description of a given VCP feature code.",5,{"inputs":[{"name":"self"},{"name":"featurecode"}],"output":{"generics":["descriptor"],"name":"option"}}]],"paths":[[3,"Descriptor"],[4,"TableInterpretation"],[4,"ValueInterpretation"],[4,"ValueType"],[4,"Access"],[3,"Database"]]};
initSearch(searchIndex);
