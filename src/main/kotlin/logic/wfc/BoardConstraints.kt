package logic.wfc

import entity.board.Board
import entity.board.Word
import common.constant.Settings
import entity.board.Direction

internal class BoardConstraints (private val board: Board) {

    private fun rowOrColToSequence(fullSequence: List<String>, start: Int, end: Int): String {
        // start at current position and look for a word from there
        val subSequence = fullSequence.subList(start, end)
        var subSequenceLastIndex = subSequence.indexOfFirst { it == "+" }
        subSequenceLastIndex = if (subSequenceLastIndex == -1) end - start else subSequenceLastIndex

        val sequence = subSequence.subList(0, subSequenceLastIndex)

        return sequence.joinToString("")
    }

    private fun findSequenceInDictionary(sequence: String): Set<Word> {
        if (!sequence.contains(" ")) return emptySet()          // in case the sequence is part of word
        return board.dictionary.findWordsFromSequence(sequence) ?: emptySet()
    }

    fun findWordsFromPositionLTR(row: Int, col: Int): Set<Word> {
        // Take current row only it should have the same index as row
        val wholeRowSequence: List<String> = board.board.filterIndexed { index, cell ->
            index / Settings.COLS == row
        }

        val sequence = rowOrColToSequence(wholeRowSequence, col, Settings.COLS)
        return findSequenceInDictionary(sequence)
    }

    fun findWordsFromPositionRTL(row: Int, col: Int): Set<Word> {
//        if (row == 0  && col == 3 ) {
//            println()
//        }
        // Take current row only it should have the same index as row
        val wholeRowSequence: List<String> = board.board.filterIndexed { index, cell ->
            index / Settings.COLS == row
        }.reversed()

        val sequence = rowOrColToSequence(wholeRowSequence, Settings.COLS - col - 1, Settings.COLS)
        return findSequenceInDictionary(sequence)
    }

    fun findWordsFromPositionTTB(row: Int, col: Int): Set<Word> {
        // Take current row only it should have the same index as row
        val wholeRowSequence: List<String> = board.board.filterIndexed { index, cell ->
            index % Settings.COLS == col
        }

        val sequence = rowOrColToSequence(wholeRowSequence, row, Settings.ROWS)
        return findSequenceInDictionary(sequence)
    }

    fun findWordsFromPositionBTT(row: Int, col: Int): Set<Word> {
        // Take current row only it should have the same index as row
        val wholeRowSequence: List<String> = board.board.filterIndexed { index, cell ->
            index % Settings.COLS == col
        }.reversed()

        val sequence = rowOrColToSequence(wholeRowSequence, Settings.ROWS - row - 1, Settings.ROWS)
        return findSequenceInDictionary(sequence)
    }
}