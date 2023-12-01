package aoc

enum InputMode {
  case Test(subproblemNo: Int)
  case Normal
}

trait Problem(problemNo: Int, inputMode: InputMode, run: Vector[String] => Unit) {
  def name: String = {
    val fileName = inputMode match {
      case InputMode.Test(subProblemNo) => s"${problemNo}_${subProblemNo}_test.txt"
      case InputMode.Normal => s"${problemNo}_normal.txt"
    }
    
    s"data/$fileName"
  }
  
  def main(args: Array[String]): Unit = {
    val source = io.Source.fromFile(name)
    val data = source.getLines().toVector
    source.close()
    run(data)
  }
}
