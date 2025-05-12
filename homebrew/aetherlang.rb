class Aetherlang < Formula
  desc "A cloud-native programming language"
  homepage "https://github.com/iamvirul/Aetherlang"
  version "1.1.0"
  license "MIT"

  url "https://github.com/iamvirul/Aetherlang/releases/download/v1.1.0/aeth" # Direct download URL
  sha256 "0019dfc4b32d63c1392aa264aed2253c1e0c2fb09216f8e2cc269bbfb8bb49b5" # Replace with the actual SHA-256 checksum

  def install
    bin.install "aeth"
    # There is no aethc binary in the release
  end

  test do
    # Add a simple test here to verify that Aetherlang is working
    system "#{bin}/aeth", "--version"
  end
end