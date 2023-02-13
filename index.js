const json_bigint = require('.')

let json_raw = '{ "big_intValue": 9223372036854775807, "in": {"in": 9007199254740991 } }';
function main(){
    console.log('json-bigint', json_bigint.parse(json_raw))
}
main()
