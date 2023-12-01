package entity

import common.utils.DictionaryUtils
import common.utils.FileReader
import entity.board.Word

class Dictionary {
    private val fileReader = FileReader("/sk.txt")
    val dictionary: Map<String, Int> = DictionaryUtils().toMap(
        fileReader.readFile()
    )
    val usedWords: MutableSet<Word> = mutableSetOf()

    fun excludeUsedWord(word: Word) = usedWords.add(word)

    fun findWordsFromSequence(sequence: String): Set<Word>? {
        if (sequence.length <= 2) return null

        val availableWords: Set<Word> = dictionary
            .filter { it.key.length == sequence.length }
            .filter { entry ->
                var matches = true

                entry.key.forEachIndexed { index, c ->
                    if (matches && sequence[index] != ' ')
                        matches = (sequence[index] == c)
                }

                matches
            }.map { Word(it.key, it.value) }
            .filterNot { usedWords.find { excWord -> excWord.word == it.word } != null }
            .toHashSet()

        return availableWords
    }

}