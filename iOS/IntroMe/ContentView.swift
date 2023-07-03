//
//  ContentView.swift
//  IntroMe
//
//  Created by Rodion Kuskov on 6/7/23.
//

import SwiftUI

struct ContentView: View {
    @EnvironmentObject var store: Store

    var body: some View {
        VStack {
            Button("Load") {
                print("AASd")
                
            }
        }
        .padding()
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}
