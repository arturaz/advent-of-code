package aoc

object Solution2_2 {
  case class MinCubes(red: Int, blue: Int, green: Int) {
    def update(draw: Map[Solution2.BallColor, Int]): MinCubes = {
      MinCubes(
        red = math.max(red, draw.getOrElse(Solution2.BallColor.Red, 0)),
        blue = math.max(blue, draw.getOrElse(Solution2.BallColor.Blue, 0)),
        green = math.max(green, draw.getOrElse(Solution2.BallColor.Green, 0))
      )
    }

    def power: Int = red * blue * green
  }

  def run(data: Vector[String]): String = {
    val result = data.iterator.map(Solution2.parseGame).map { game =>
      game.draws.foldLeft(MinCubes(0, 0, 0))((minCubes, draw) => minCubes.update(draw))
    }.map(_.power).sum
    result.toString
  }
}

object _2_2_Test extends Problem(2, InputMode.Test(1), Solution2_2.run)
object _2_2_Normal extends Problem(2, InputMode.Normal, Solution2_2.run)