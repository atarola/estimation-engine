import { useSelector } from 'react-redux'

export function Header() {
  return (
    <nav className="navbar is-dark" role="navigation" aria-label="main navigation">
      <div className="navbar-brand">
        <div className="navbar-item">
          <span className="fa-stack fa-lg">
            <i className="fa fa-square fa-stack-2x"></i>
            <i className="fa fa-motorcycle fa-stack-1x fa-inverse"></i>
          </span>
          <h1>Estimation Engine</h1>
        </div>

        <Topic />
      </div>
    </nav>
  );
}

function Topic() {
  let id = useSelector((store) => store.id);

  if (id === null) {
    return null;
  }

  return (
    <div className="navbar-item">
      Room Link: &nbsp;
      <span className="is-family-monospace">
        {window.location.href}
      </span>
    </div>
  );
}
