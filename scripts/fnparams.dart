String rtype(String ctype) {
  switch (ctype) {
    case "int64_t":
      return "*mut i64";
    case "jdouble":
      return "*mut f64";
    default:
      return "?";
  }
}

main(List<String> args) {
  var params = """
    int64_t *M,
    int64_t *N,
    jdouble *ALPHA,
    jdouble *A,
    int64_t *LDA,
    jdouble *X,
    int64_t *INCX,
    jdouble *BETA,
    jdouble *Y,
    int64_t *INCY
  """;
  params.trim().split(",").forEach((s) {
    var param = s.trim().split(" \*");
    var type = rtype(param[0].trim());
    var name = param[1].trim();
    print("$name: $type,");
  });
}
