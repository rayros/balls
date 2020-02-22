import * as ghpages from 'gh-pages';

ghpages.publish('target/deploy', {
  branch: 'master',
  repo: 'https://' + process.env.GH_TOKEN + '@github.com/studioLaCosaNostra/studioLaCosaNostra.github.io',
  dest: 'games/lines'
}, function (error) {
  if (error) {
    console.error(error);
    process.exit(1);
  }
});
