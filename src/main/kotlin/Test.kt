import common.constant.Dictionary
import common.utils.FileReader
import java.util.ResourceBundle

var file = FileReader("/sk.txt")

fun main(args: Array<String>) {
    readFile()
//    searchInFile("rudysbooks")
}

fun readFile() {
    val start = System.currentTimeMillis()
    file.readFile()
    val end = System.currentTimeMillis()

    println("Reading file took ${end - start}[ms]")
}

fun searchInFile(word: String) {
    val start = System.currentTimeMillis()

    val lines = file.readFile()
    val line = lines.firstOrNull { it.startsWith(word) }

    val end = System.currentTimeMillis()
    println("Searching for ${word} ends with ${line} took ${end - start}[ms]")

}
