//
//  Experience.swift
//  IntroMe
//
//  Created by Rodion Kuskov on 6/7/23.
//

import Foundation

struct Experience: Identifiable {
    let id: UUID = .init()
    let name: String
    let description: String
}
