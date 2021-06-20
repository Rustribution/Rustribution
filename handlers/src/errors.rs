custom_error! {pub HandlerError
  InvalidSecret{msg:String}="invalid secret: {msg}",
  HamcError{msg:String}="Hmac error: {msg}",
  InvalidBase64="invalid base64 string",
  InvalidEndcodeJSON="invalid Encode JSON",
}
