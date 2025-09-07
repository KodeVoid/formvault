import { useEffect, useState } from "react";

export const Issues = () => {
  const [issues, setIssues] = useState([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(null);

  useEffect(() => {
    const fetchIssues = async () => {
      try {
        setLoading(true);
        setError(null);

        const response = await fetch(
          "https://api.github.com/repos/kodevoid/formvault/issues?per_page=20"
        );

        if (!response.ok) {
          throw new Error(`Failed to fetch issues: ${response.status}`);
        }

        const data = await response.json();

        const formatted = data.map((issue) => ({
          id: issue.id,
          title: issue.title,
          url: issue.html_url,
          labels: issue.labels.map((l) => l.name),
          state: issue.state, // open/closed
        }));

        setIssues(formatted);
      } catch (err) {
        setError(err.message);
      } finally {
        setLoading(false);
      }
    };

    fetchIssues();
  }, []);

  return (
    <section id="issues" className="p-4">
      <h2 className="text-center text-3xl font-bold mb-2">Latest Issues</h2>

      {loading && <p>Loading issues…</p>}
      {error && <p className="text-red-500">Error: {error}</p>}

      {!loading && !error && issues.length === 0 && (
        <p className="text-gray-500">No issue found</p>
      )}

      {!loading && !error && issues.length > 0 && (
        <ul className="space-y-2">
          {issues.map((issue) => (
            <li key={issue.id} className="p-2 border rounded">
              <a
                href={issue.url}
                target="_blank"
                rel="noopener noreferrer"
                className="text-blue-600 font-medium"
              >
                {issue.title}
              </a>
              <div className="text-sm text-gray-600 flex gap-2">
                <span>Labels: {issue.labels.join(", ") || "none"}</span>
                <span>
                  State: {issue.state === "closed" ? " Done" : " Open"}
                </span>
              </div>
            </li>
          ))}
        </ul>
      )}
    </section>
  );
};
