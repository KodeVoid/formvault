export default function Home() {
  return (
    <div className="min-h-screen bg-black text-white">
      <section id="home" className="relative overflow-hidden">
        <div className="absolute inset-0 bg-black/20 backdrop-blur-sm"></div>
        
        {/* Hero Section */}
        <div className="relative max-w-7xl mx-auto px-8 py-20">
          <div className="text-center mb-16">
            <div className="inline-flex items-center bg-white/10 border border-white/30 rounded-full px-6 py-3 mb-8">
              <span className="text-gray-300 text-sm font-medium">🔒 Privacy-First Form Handling</span>
            </div>
            
            <h1 className="text-6xl md:text-7xl font-bold mb-8 text-white">
              Welcome to FormVault
            </h1>
            
            <p className="text-xl md:text-2xl text-gray-300 max-w-4xl mx-auto mb-12 leading-relaxed">
              The developer-first form handling service with <span className="text-gray-100 font-semibold">zero-knowledge encryption</span> and 
              <span className="text-gray-100 font-semibold"> privacy at its core</span>
            </p>
            
            <h2 className="text-3xl md:text-4xl font-semibold mb-12 text-white">
              What would you like to do?
            </h2>
          </div>

          {/* Action Cards */}
          <div className="grid grid-cols-1 md:grid-cols-3 gap-8 mb-16">
            <div className="group bg-white/5 backdrop-blur-sm border border-white/10 rounded-xl p-8 hover:bg-white/10 hover:border-white/20 transition-all duration-300 transform hover:scale-105">
              <div className="text-4xl mb-4">🤝</div>
              <h3 className="text-xl font-semibold mb-4">Contribute</h3>
              <p className="text-gray-300 mb-6">
                Join our open-source community and help build the future of secure form handling
              </p>
              <a 
                href="#contributions" 
                className="inline-flex items-center bg-white/20 hover:bg-white/30 px-6 py-3 rounded-lg transition-all duration-300 font-medium"
              >
                Get Started
                <svg className="w-4 h-4 ml-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5l7 7-7 7" />
                </svg>
              </a>
            </div>

            <div className="group bg-white/5 backdrop-blur-sm border border-white/10 rounded-xl p-8 hover:bg-white/10 hover:border-white/20 transition-all duration-300 transform hover:scale-105">
              <div className="text-4xl mb-4">🚀</div>
              <h3 className="text-xl font-semibold mb-4">Join Waitlist</h3>
              <p className="text-gray-300 mb-6">
                Be among the first to experience secure, zero-knowledge form handling
              </p>
              <a 
                href="#cta" 
                className="inline-flex items-center bg-white/20 hover:bg-white/30 px-6 py-3 rounded-lg transition-all duration-300 font-medium"
              >
                Sign Up Now
                <svg className="w-4 h-4 ml-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5l7 7-7 7" />
                </svg>
              </a>
            </div>

            <div className="group bg-white/5 backdrop-blur-sm border border-white/10 rounded-xl p-8 hover:bg-white/10 hover:border-white/20 transition-all duration-300 transform hover:scale-105">
              <div className="text-4xl mb-4">💬</div>
              <h3 className="text-xl font-semibold mb-4">Ask Questions</h3>
              <p className="text-gray-300 mb-6">
                Have questions about FormVault? We're here to help you get started
              </p>
              <a 
                href="#contact" 
                className="inline-flex items-center bg-white/20 hover:bg-white/30 px-6 py-3 rounded-lg transition-all duration-300 font-medium"
              >
                Contact Us
                <svg className="w-4 h-4 ml-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5l7 7-7 7" />
                </svg>
              </a>
            </div>
          </div>

          {/* Features Preview */}
          <div className="bg-white/5 backdrop-blur-sm border border-white/10 rounded-2xl p-12 mb-16">
            <h3 className="text-3xl font-bold text-center mb-8 text-white">
              Why Choose FormVault?
            </h3>
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
              <div className="text-center">
                <div className="bg-white/10 w-16 h-16 rounded-full flex items-center justify-center mx-auto mb-4">
                  <span className="text-2xl">🔐</span>
                </div>
                <h4 className="font-semibold mb-2">Zero-Knowledge</h4>
                <p className="text-gray-300 text-sm">End-to-end encryption ensures your data stays private</p>
              </div>
              <div className="text-center">
                <div className="bg-white/10 w-16 h-16 rounded-full flex items-center justify-center mx-auto mb-4">
                  <span className="text-2xl">⚡</span>
                </div>
                <h4 className="font-semibold mb-2">Developer-First</h4>
                <p className="text-gray-300 text-sm">RESTful API and SDKs for seamless integration</p>
              </div>
              <div className="text-center">
                <div className="bg-white/10 w-16 h-16 rounded-full flex items-center justify-center mx-auto mb-4">
                  <span className="text-2xl">🛡️</span>
                </div>
                <h4 className="font-semibold mb-2">Secure by Design</h4>
                <p className="text-gray-300 text-sm">Built-in validation and sanitization</p>
              </div>
              <div className="text-center">
                <div className="bg-white/10 w-16 h-16 rounded-full flex items-center justify-center mx-auto mb-4">
                  <span className="text-2xl">🌐</span>
                </div>
                <h4 className="font-semibold mb-2">Cross-Platform</h4>
                <p className="text-gray-300 text-sm">JavaScript, Python, and Rust SDKs</p>
              </div>
            </div>
          </div>

          {/* About Section */}
          <div id="about" className="text-center mb-16">
            <h3 className="text-4xl font-bold mb-8">About FormVault</h3>
            <div className="max-w-4xl mx-auto">
              <p className="text-lg text-gray-300 mb-8 leading-relaxed">
                FormVault is revolutionizing how developers handle form submissions with a focus on 
                <span className="text-gray-100 font-semibold"> privacy and security</span>. Our zero-knowledge architecture 
                ensures that even we cannot read your users' data without the encryption key.
              </p>
              
              <div className="grid grid-cols-1 md:grid-cols-2 gap-8 mb-12">
                <div className="border border-white/10 rounded-xl p-6">
                  <h4 className="text-xl font-semibold mb-4 text-gray-100">🎯 Our Mission</h4>
                  <p className="text-gray-300">
                    To provide developers with secure, privacy-first form handling tools that protect user data 
                    while maintaining ease of use and integration.
                  </p>
                </div>
                <div className="bg-white/5 border border-white/10 rounded-xl p-6">
                  <h4 className="text-xl font-semibold mb-4 text-gray-100">🌟 Our Vision</h4>
                  <p className="text-gray-300">
                    A web where form data is handled with the highest standards of privacy and security, 
                    making data breaches and privacy violations a thing of the past.
                  </p>
                </div>
              </div>

              <div className="bg-white/5 backdrop-blur-sm border border-white/10 rounded-xl p-8">
                <h4 className="text-2xl font-semibold mb-4 text-gray-100">Help Shape FormVault's Future</h4>
                <p className="text-gray-300 mb-6">
                  Your feedback is crucial in building the best form handling service. Take our survey and 
                  let us know what features matter most to you.
                </p>
                <a
                  href="https://docs.google.com/forms/d/e/1FAIpQLSdSu7_cdfzU7Pye1VclW3jBOWC1QfwU3ZC6HYx0ifFYyQhOGg/viewform"
                  target="_blank"
                  rel="noopener noreferrer"
                  className="inline-flex items-center bg-white/20 hover:bg-white/30 px-8 py-4 rounded-xl transition-all duration-300 transform hover:scale-105 shadow-lg hover:shadow-xl font-semibold"
                >
                  <span className="text-lg mr-3">📋</span>
                  Fill Out Survey Form
                  <svg className="w-5 h-5 ml-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14" />
                  </svg>
                </a>
              </div>
            </div>
          </div>

          {/* CTA Section */}
          <div id="cta" className="text-center">
            <div className="bg-white/5 backdrop-blur-sm border border-white/10 rounded-2xl p-12">
              <h3 className="text-4xl font-bold mb-6">Ready to Get Started?</h3>
              <p className="text-xl mb-8 max-w-2xl mx-auto">
                Join the waitlist today and be among the first to experience the future of secure form handling
              </p>
              <div className="flex flex-col sm:flex-row gap-4 justify-center">
                <button className="bg-white text-gray-900 hover:bg-gray-100 px-8 py-4 rounded-xl transition-colors font-semibold text-lg">
                  Join Waitlist
                </button>
                <button className="border-2 border-white hover:bg-white hover:text-gray-900 px-8 py-4 rounded-xl transition-colors font-semibold text-lg">
                  View Documentation
                </button>
              </div>
            </div>
          </div>
        </div>
      </section>
    </div>
  );
}