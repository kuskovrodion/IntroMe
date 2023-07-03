//
//  Profile.swift
//  IntroMe
//
//  Created by Rodion Kuskov on 6/7/23.
//

import SwiftUI

struct Profile: Identifiable {
    let id: UUID = .init()
    let name: String
    let position: String
    let description: String
    let image: UIImage?
    let experience: [Experience]
}
