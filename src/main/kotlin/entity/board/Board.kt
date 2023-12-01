package entity.board

import common.constant.Settings
import entity.Dictionary
import kotlin.random.Random

class Board() {
    val dictionary: Dictionary = Dictionary()
    val board = MutableList<String>(Settings.ROWS * Settings.COLS) { " " }
        get() = field

    fun initNotRandom(solutionWord: Word) {
        board[2] = "a"
        board[4] = "+"
        board[6] = "+"
        board[10] = "+"
        board[16] = "+"
        board[25] = "+"
        board[30] = "+"
        board[35] = "+"
    }

    fun randomAssign(solutionWord: Word) {
        var currentIdx = 0
        while (currentIdx < solutionWord.word.length) {
            solutionWord.word.forEach {
                if (currentIdx < solutionWord.word.length) {
                    var lastIndex = if (currentIdx == 0) 0 else board.lastIndexOf("${solutionWord.word[currentIdx - 1]}")
                    val idx = Random.nextInt(lastIndex, board.size - (solutionWord.word.length - currentIdx))
                    if (board[idx] == " ") {
//                        board[idx] = "${solutionWord.word[currentIdx]}"
                        board[idx] = "+"
                        currentIdx++
                    }
                }
            }
        }
    }


}