pipeline {
    agent {
        docker {
            image 'rust:1.51'
        }
    }
    stages {
        stage('Test') {
            steps {
                echo 'Testing...'
                sh 'cargo test'
            }
        }
        stage('Build') {
            steps {
                echo 'Building...'
                sh 'cat Cargo.toml'
                sh 'cargo build --release'
                sh 'file target/release/basic_wake_on_lan_api'
                sh 'ldd target/release/basic_wake_on_lan_api'
                sh 'readelf -e target/release/basic_wake_on_lan_api'
            }
        }
    }
}