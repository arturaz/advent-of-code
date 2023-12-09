package aoc

object Solution9 {
  def parse(data: Vector[String]): Vector[Vector[Long]] =
    data.map(_.split("\\s+").iterator.map(_.toLong).toVector)

  def diffs(data: Vector[Long]): Vector[Long] =
    data.sliding(2).map { case Vector(a, b) => b - a }.toVector

  def derive(data: Vector[Long]): Vector[Vector[Long]] = {
    val b = Vector.newBuilder[Vector[Long]]
    var current = diffs(data)
    while (!current.forall(_ == 0)) {
      b += current
      current = diffs(current)
    }
    b.result()
  }

  def predict(
    source: Vector[Long], data: Vector[Vector[Long]], forwards: Boolean
  ): Vector[Long] = {
    def predictNext(row: Vector[Long], lastPredicted: Long) =
      if (forwards) row.last + lastPredicted
      else row.head - lastPredicted

    val predictions = data.dropRight(1).foldRight(Vector(
      if (forwards) data.last.last else data.last.head
    )) { case (row, acc) =>
      val nextPredicted = predictNext(row, acc.last)
      acc :+ nextPredicted
    }
    val lastPredicted = predictNext(source, predictions.last)
    predictions :+ lastPredicted
  }

  def run(data: Vector[String], forwards: Boolean): String = {
    val parsed = parse(data)
    val result = parsed.map { parsed =>
      val d = derive(parsed)
      val p = predict(parsed, d, forwards)
      println(s"$parsed -> [\n  ${d.mkString(",\n  ")}\n] -> $p")
      p.last
    }.sum

    result.toString
  }

  def run1(data: Vector[String]): String = run(data, forwards = true)
  def run2(data: Vector[String]): String = run(data, forwards = false)
}

object _9_1_Test1 extends Problem(9, InputMode.Test(1), Solution9.run1)
object _9_1_Normal extends Problem(9, InputMode.Normal, Solution9.run1)

object _9_2_Test1 extends Problem(9, InputMode.Test(1), Solution9.run2)
object _9_2_Normal extends Problem(9, InputMode.Normal, Solution9.run2)