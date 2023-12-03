package aoc

/**
 * Given a string like "zoneight" and a replacement list like [("eight", "8"), ("one", "1")] and an index like 1 it
 * should return true if the string matches the replacement at the given index.
 */
def matchesAt(input: String, replacements: Vector[(String, String)], index: Int): Option[(String, String)] = {
  replacements.find { case (from, _) =>
    index + from.length <= input.length && input.substring(index, index + from.length) == from
  }
}

def buildNewWithReplacements(input: String, replacements: Vector[(String, String)]): String = {
  val current = new StringBuilder()

  for (index <- (0 until input.length)) {
    matchesAt(input, replacements, index) match {
      case Some((_, to)) =>
        current.append(to)
      case None =>
        if (input(index).isDigit) current.append(input(index))
    }
  }

  current.result()
}

def solution_1_2(data: Vector[String]): Unit = {
  val replacements = Vector(
    "three" -> "3",
    "seven" -> "7",
    "eight" -> "8",
    "nine" -> "9",
    "four" -> "4",
    "five" -> "5",
    "one" -> "1",
    "two" -> "2",
    "six" -> "6",
  )

  val processed = data.zipWithIndex.map { case (str, idx) =>
    val replaced = buildNewWithReplacements(str, replacements)
    println(s"#$idx: $str -> $replaced")
    replaced
  }

  println()
  solution1(processed)
}

object _1_2_Test extends Problem(1, InputMode.Test(2), solution_1_2)
object _1_2_Normal extends Problem(1, InputMode.Normal, solution_1_2)
