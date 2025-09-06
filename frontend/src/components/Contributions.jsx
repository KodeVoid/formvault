import { useState, useEffect } from "react";

const Contributions = ({ owner = "kodevoid", repo = "formvault" }) => {
  const [contributors, setContributors] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);

  useEffect(() => {
    const fetchContributors = async () => {
      setLoading(true);
      setError(null);
      try {
        const response = await fetch(
          `https://api.github.com/repos/${owner}/${repo}/contributors`
        );

        if (!response.ok) {
          throw new Error(`Failed to fetch contributors: ${response.status}`);
        }

        const data = await response.json();

        // ✅ Filter out bots
        const humans = data.filter((contributor) => contributor.type !== "Bot");

        setContributors(humans.slice(0, 20)); // Top 20 human contributors
      } catch (err) {
        setError(err.message);
      } finally {
        setLoading(false);
      }
    };

    fetchContributors();
  }, [owner, repo]);

  if (loading) {
    return (
      <div className="min-h-screen text-white p-8 flex justify-center items-center">
        <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-white"></div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="min-h-screen text-white p-8 flex justify-center items-center">
        <div className="text-center">
          <p className="text-red-400 text-lg mb-2">
            Error loading contributors
          </p>
          <p className="text-gray-300">{error}</p>
          <p className="text-sm text-gray-400 mt-4">
            Ensure the repository{" "}
            <strong>
              {owner}/{repo}
            </strong>{" "}
            exists and is public.
          </p>
        </div>
      </div>
    );
  }

  return (
    <div className="min-h-screen text-white p-8">
      <div className="max-w-6xl mx-auto">
        <div className="text-center mb-12">
          <h2 className="text-4xl font-bold mb-4">Contributors</h2>
          <p className="text-gray-300 text-lg">
            Thank you to all the amazing people who have contributed to{" "}
            <span className="text-blue-400 font-semibold">
              <a
                href={`https://github.com/${owner}/${repo}`}
                target="_blank"
                rel="noopener noreferrer"
              >
                {owner}/{repo}
              </a>
            </span>
          </p>
        </div>

        {contributors.length === 0 ? (
          <div className="text-center text-gray-400">
            <p>No human contributors found for this repository.</p>
          </div>
        ) : (
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6">
            {contributors.map((contributor) => (
              <div
                key={contributor.id}
                className="bg-gray-800 rounded-lg p-6 hover:bg-gray-700 transition-colors border border-gray-700 hover:border-gray-600"
              >
                <div className="flex flex-col items-center">
                  <img
                    src={contributor.avatar_url}
                    alt={`${contributor.login}'s avatar`}
                    className="w-20 h-20 rounded-full mb-4 border-2 border-gray-600"
                  />
                  <h3 className="text-xl font-semibold mb-2">
                    {contributor.login}
                  </h3>
                  <p className="text-green-400 font-bold text-lg mb-4">
                    {contributor.contributions} contributions
                  </p>
                  <a
                    href={contributor.html_url}
                    target="_blank"
                    rel="noopener noreferrer"
                    className="bg-blue-600 hover:bg-blue-700 px-4 py-2 rounded-lg transition-colors text-sm font-medium"
                  >
                    View Profile
                  </a>
                </div>
              </div>
            ))}
          </div>
        )}

        <div className="mt-12 text-center">
          <a
            href={`https://github.com/${owner}/${repo}`}
            target="_blank"
            rel="noopener noreferrer"
            className="inline-flex items-center hover:bg-opacity-70 px-6 py-3 rounded-lg transition-colors border border-opacity-30 hover:border-opacity-50"
          >
            <svg
              className="w-5 h-5 mr-2"
              fill="currentColor"
              viewBox="0 0 24 24"
            >
              <path d="M12 0C5.374 0 0 5.373 0 12 0 17.302 3.438 21.8 8.207 23.387c.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23A11.509 11.509 0 0112 5.803c1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576C20.566 21.797 24 17.3 24 12c0-6.627-5.373-12-12-12z" />
            </svg>
            View Repository
          </a>
        </div>
      </div>
    </div>
  );
};

export default Contributions;
