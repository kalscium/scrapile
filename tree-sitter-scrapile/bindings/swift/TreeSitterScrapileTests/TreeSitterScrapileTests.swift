import XCTest
import SwiftTreeSitter
import TreeSitterScrapile

final class TreeSitterScrapileTests: XCTestCase {
    func testCanLoadGrammar() throws {
        let parser = Parser()
        let language = Language(language: tree_sitter_scrapile())
        XCTAssertNoThrow(try parser.setLanguage(language),
                         "Error loading Scrapile grammar")
    }
}