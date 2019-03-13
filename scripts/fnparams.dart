String rtype(String ctype) {
  switch (ctype) {
    case "int64_t":
      return "*mut i64";
    case "jdouble":
      return "*mut f64";
    case "jchar":
      return "*mut c_char";
    default:
      return "?";
  }
}

main(List<String> args) {
  var params = """
    int64_t *N,
    jdouble *A,
    int64_t *LDA,
    int64_t *IPIV,
    jdouble *WORK,
    int64_t *LWORK,
    int64_t *INFO
  """;
  params.trim().split(",").forEach((s) {
    var param = s.trim().split(" \*");
    var type = rtype(param[0].trim());
    var name = param[1].trim().toUpperCase();
    print("$name: $type,");
  });
}
