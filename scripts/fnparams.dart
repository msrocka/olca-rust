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
    int64_t *n,
    int64_t *nrhs,
    jdouble *A,
    int64_t *lda,
    int64_t *ipiv,
    jdouble *B,
    int64_t *ldb,
    int64_t *info
  """;
  params.trim().split(",").forEach((s) {
    var param = s.trim().split(" \*");
    var type = rtype(param[0].trim());
    var name = param[1].trim().toUpperCase();
    print("$name: $type,");
  });
}
