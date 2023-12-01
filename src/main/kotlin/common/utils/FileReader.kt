package common.utils

class FileReader(private val filePath: String) {

    constructor(filePath: String, isResource: Boolean?) : this(filePath)

    fun readFile(): List<String> {
        val resource = this::class.java.getResourceAsStream(filePath)

        val lines = resource?.bufferedReader()?.readLines()
        resource?.close()

        return lines ?: emptyList()
    }

//    fun closeFile(): Boolean {
//        this::class.java.getResourceAsStream(filePath)
//    }
}