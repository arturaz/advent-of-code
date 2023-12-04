package aoc

object Solution2 {
  enum BallColor {
    case Green, Blue, Red
  }

  case class Game(no: Int, draws: Vector[Map[BallColor, Int]])

  // Game 1: 7 green, 4 blue, 3 red; 4 blue, 10 red, 1 green; 1 blue, 9 red
  def parseGame(line: String): Game = {
    val Array(gameStr, drawsStr) = line.split(": ")
    val game = gameStr.split(" ").last.toInt
    val draws = drawsStr.split("; ").iterator.map { setStr =>
      setStr.split(", ").iterator.map(_.split(" ")).map {
        case Array(nStr, "green") => BallColor.Green -> nStr.toInt
        case Array(nStr, "blue") => BallColor.Blue -> nStr.toInt
        case Array(nStr, "red") => BallColor.Red -> nStr.toInt
        case other => throw new Exception(s"Unexpected input: ${other.toVector}")
      }.toMap
    }.toVector

    Game(game, draws)
  }

  def validate(game: Game): Boolean = {
    game.draws.forall { draw =>
      val red = draw.getOrElse(BallColor.Red, 0)
      val green = draw.getOrElse(BallColor.Green, 0)
      val blue = draw.getOrElse(BallColor.Blue, 0)
      red <= 12 && green <= 13 && blue <= 14
    }
  }

  def run(data: Vector[String]): String = {
    val result = data.iterator.map(parseGame).filter(validate).map(_.no).sum
    result.toString
  }
}

object _2_1_Test extends Problem(2, InputMode.Test(1), Solution2.run)
object _2_1_Normal extends Problem(2, InputMode.Normal, Solution2.run)
