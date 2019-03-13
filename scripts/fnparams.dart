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
    jchar *TRANSA,
    jchar *TRANSB,
    int64_t *M,
    int64_t *N,
    int64_t *K,
    jdouble *ALPHA,
    jdouble *A,
    int64_t *LDA,
    jdouble *B,
    int64_t *LDB,
    jdouble *BETA,
    jdouble *C,
    int64_t *LDC
  """;
  params.trim().split(",").forEach((s) {
    var param = s.trim().split(" \*");
    var type = rtype(param[0].trim());
    var name = param[1].trim();
    print("$name: $type,");
  });
}
