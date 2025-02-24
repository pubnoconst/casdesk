import scala.quoted.*
import java.nio.file.{Files, Paths}
import java.nio.charset.StandardCharsets

private inline def compileTimeRender(
    inline path: String,
    inline replacements: Map[String, String]
): String =
  ${ compileTimeRenderImpl('path, 'replacements) }

private def compileTimeRenderImpl(
    pathExpr: Expr[String],
    replacementsExpr: Expr[Map[String, String]]
)(using Quotes): Expr[String] =
  val path = pathExpr.valueOrAbort
  val filePath = Paths.get("../assets/mockups", path)

  // Read file at compile-time
  val template =
    try Files.readString(filePath, StandardCharsets.UTF_8)
    catch
      case e: Exception =>
        throw new RuntimeException(s"Failed to read file: $filePath", e)

  // Replace placeholders
  val pattern = "\\b(__[A-Za-z0-9_]+)\\b".r
  val replacements = replacementsExpr.valueOrAbort

  val rendered = pattern.replaceAllIn(
    template,
    m => replacements.getOrElse(m.matched, m.matched)
  )

  Expr(rendered)


