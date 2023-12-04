package aoc

def solution1(data: Vector[String]): String = {
  val result = data.iterator.zipWithIndex.map { case (str, idx) =>
    val digits = str.filter(_.isDigit).map(_.asDigit).toArray
    val first = digits.head
    val last = digits.last
    val result = first * 10 + last
    println(s"#$idx: $str -> $first$last = $result")
    result
  }.sum

  result.toString
}

object _1_1_Test extends Problem(1, InputMode.Test(1), solution1)
object _1_1_Normal extends Problem(1, InputMode.Normal, solution1)