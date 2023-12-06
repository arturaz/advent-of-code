package aoc

import scala.collection.immutable.NumericRange

object Solution6 {
  def calculate(timeRace: Double, timeHeld: Double): Double =
    (timeRace - timeHeld) * timeHeld

  def calculateHeld(timeRace: Double, distance: Double): NumericRange.Inclusive[Long] = {
    val a = timeRace / 2 - math.sqrt(-4 * distance + timeRace * timeRace) / 2
    val b = timeRace / 2 + math.sqrt(-4 * distance + timeRace * timeRace) / 2

    println(s"$a -> ${calculate(timeRace, a)}")
    println(s"$b -> ${calculate(timeRace, b)}")

    val ceiled = a.ceil.toLong
    val floored = b.floor.toLong
    (if (a == ceiled) ceiled + 1 else ceiled) to (if (b == floored) floored - 1 else floored)
  }

  def solve(timeRace: BigDecimal, distance: BigDecimal): Long = {
    val range = calculateHeld(timeRace.toDouble, distance.toDouble)
//    println(s"range: $range -> ${range.map(calculate(timeRace, _))}")
    range.end - range.start + 1
  }

  case class Entry(timeRace: BigInt, distance: BigInt)

  def parse(data: Vector[String]): Vector[Entry] = {
    val times = data(0).split(':')(1).trim.split("\\s+").map(BigInt(_))
    val distances = data(1).split(':')(1).trim.split("\\s+").map(BigInt(_))
    times.iterator.zip(distances.iterator).map(Entry.apply).toVector
  }

  def run(entries: Vector[Entry]): String = {
    println(entries)
    val result = entries.iterator.map(e => solve(BigDecimal(e.timeRace), BigDecimal(e.distance))).toVector
    println(result)
    result.product.toString
  }

  def run1(data: Vector[String]): String = {
    val entries = parse(data)
    run(entries)
  }

  def run2(data: Vector[String]): String = {
    val entries = parse(data.map(_.replaceAll("\\s+", "")))
    run(entries)
  }
}

object _6_1_Test extends Problem(6, InputMode.Test(1), Solution6.run1)
object _6_1_Normal extends Problem(6, InputMode.Normal, Solution6.run1)

object _6_2_Test extends Problem(6, InputMode.Test(1), Solution6.run2)
object _6_2_Normal extends Problem(6, InputMode.Normal, Solution6.run2)
