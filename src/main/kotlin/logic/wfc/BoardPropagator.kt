package logic.wfc

import entity.board.Board
import entity.board.Direction
import entity.board.Word
import common.constant.Settings

class BoardPropagator(val board: Board) {
    private val waveFunctionCollapse = WaveFunctionCollapse(board)

    fun fillSingleWord() {
        val superPosition = waveFunctionCollapse.selectPossibleLocation()
        val directionalWord = superPosition.getWord()

        println("${superPosition.row} ${superPosition.col} ${directionalWord.first} === ${directionalWord.second.word} (${directionalWord.second.occurrence})")
        if (superPosition.row == 5 && superPosition.col == 1) {
            println()
        }
        propagateWordToBoard(superPosition.row, superPosition.col, directionalWord.first, directionalWord.second)

        // remove this word from dictionary
        board.dictionary.excludeUsedWord(directionalWord.second)
    }

    fun isBoardPropagated(): Boolean {
        val propagated = waveFunctionCollapse.rankPossibleStates().filter { it != 0 }
        return propagated.isEmpty()
    }

    private fun propagateWordToBoard(row: Int, col: Int, direction: Direction, word: Word) {
        // TODO: skontroluj vypocty
        word.word.forEachIndexed { idx, c ->
            when (direction) {
                Direction.LTR -> {
                    var pointer = (row * Settings.COLS) + col + idx
                    board.board[pointer] = "$c"
                }

                Direction.RTL -> {
                    var pointer = ((row + 1)  * Settings.COLS) - (Settings.COLS - col + idx)
                    board.board[pointer] = "$c"
                }

                Direction.TTB -> {
                    var pointer = ((row * Settings.COLS + col) + (Settings.COLS * idx))
                    board.board[pointer] = "$c"
                }

                Direction.BTT -> {
                    var pointer = (((row + 1) * Settings.COLS) - (Settings.COLS * idx)) - (Settings.COLS - col)
                    board.board[pointer] = "$c"
                }
            }
        }
    }
}